mod app;
mod downloader;
mod ui;

use ratatui::{Terminal, backend::CrosstermBackend};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use app::{App, DownloadMode, DownloadStatus, HistoryItem};
use downloader::{download_youtube, fetch_title};
use std::{
    io,
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};
use ui::ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res_app = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res_app {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let status_ptr = Arc::new(Mutex::new(app.status.clone()));

    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok(status) = status_ptr.lock() {
            if app.status != DownloadStatus::Success && *status == DownloadStatus::Success {
                app.show_popup = true;
                app.popup_start = Some(Instant::now());
            }
            app.status = status.clone();
        }

        if let Ok(title) = rx.try_recv() {
            app.history.push(HistoryItem { title });
        }

        terminal.draw(|f| ui::<B>(f, app))?;

        if let Some(popup_time) = app.popup_start {
            if popup_time.elapsed() >= Duration::from_millis(2500) {
                app.show_popup = false;
                app.popup_start = None;
            }
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Tab => app.toggle_mode(),
                    KeyCode::Char('\n') | KeyCode::Enter => {
                        if app.url_input.is_empty() {
                            continue;
                        }
                        let url = app.url_input.clone();
                        let mode = app.mode.clone();

                        {
                            let mut status = status_ptr.lock().unwrap();
                            *status = DownloadStatus::Downloading;
                        }

                        let status_ptr_clone = Arc::clone(&status_ptr);
                        let tx_clone = tx.clone();

                        thread::spawn(move || {
                            let result =
                                download_youtube(&url, matches!(mode, DownloadMode::Audio));
                            let title_result = fetch_title(&url);

                            let mut status = status_ptr_clone.lock().unwrap();
                            *status = match result {
                                Ok(_) => {
                                    if let Ok(title) = title_result {
                                        let _ = tx_clone.send(title);
                                    }
                                    DownloadStatus::Success
                                }
                                Err(_) => DownloadStatus::Failed,
                            };
                        });

                        // app.status = DownloadStatus::Downloading; // will be updated later by thread
                    }
                    KeyCode::Backspace => {
                        app.url_input.pop();
                    }
                    KeyCode::Char(c) => app.url_input.push(c),
                    _ => {}
                }
            }
        }
    }
}
