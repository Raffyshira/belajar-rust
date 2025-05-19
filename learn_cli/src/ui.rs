use ratatui::{
    Frame,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, DownloadMode, DownloadStatus};

fn centered_rect(x: u16, y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - y) / 2),
            Constraint::Percentage(y),
            Constraint::Percentage((100 - y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - x) / 2),
            Constraint::Percentage(x),
            Constraint::Percentage((100 - x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(2),
        ])
        .split(f.area());

    let chunks_x = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let url_input = Paragraph::new(app.url_input.as_str()).block(
        Block::default()
            .title(" Youtube URL ")
            .borders(Borders::ALL),
    );
    f.render_widget(url_input, chunks[0]);

    let mode = match app.mode {
        DownloadMode::Video => " Video ( Mp4 )",
        DownloadMode::Audio => " Audio ( Mp3 )",
    };

    let mode_display =
        Paragraph::new(mode).block(Block::default().title(" Format ").borders(Borders::ALL));
    f.render_widget(mode_display, chunks_x[0]);

    let status_text = match app.status {
        DownloadStatus::Idle => Span::styled(" Idle", Style::default().fg(Color::Gray)),
        DownloadStatus::Downloading => {
            Span::styled(" Downloading", Style::default().fg(Color::Yellow))
        }
        DownloadStatus::Success => Span::styled(" Success", Style::default().fg(Color::Green)),
        DownloadStatus::Failed => Span::styled(" Failed", Style::default().fg(Color::Red)),
    };

    let status_display =
        Paragraph::new(status_text).block(Block::default().title(" Status ").borders(Borders::ALL));
    f.render_widget(status_display, chunks_x[1]);

    // Instruction line
    let instructions = Paragraph::new(Text::from(
        "[Enter] Unduh • [Tab] Ganti Mode • [Esc] Keluar",
    ))
    .style(Style::default().fg(Color::White));
    f.render_widget(instructions, chunks[3]);

    if app.show_popup {
        let popup_area = centered_rect(60, 20, f.area());

        let popup = Paragraph::new("Download selesai!")
            .style(Style::default().bg(Color::Black).fg(Color::Green))
            .alignment(Alignment::Center)
            .block(Block::default().title("Info").borders(Borders::ALL));

        f.render_widget(popup, popup_area);
    }

    // history log
    let history_block = Block::default().title(" History ").borders(Borders::ALL);
    let history_items: Vec<ListItem> = app
        .history
        .iter()
        .rev()
        .map(|h| ListItem::new(h.title.clone()))
        .collect();
    let history = List::new(history_items)
        .block(history_block)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(history, chunks[2]);
}
