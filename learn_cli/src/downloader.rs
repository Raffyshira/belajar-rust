use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn download_youtube(url: &str, audio_only: bool) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("yt-dlp");

    if audio_only {
        cmd.arg("-x").arg("--audio-format").arg("mp3");
    }

    let download_dir = dirs::download_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    cmd.arg("-P").arg(download_dir);

    cmd.arg(url).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stderr = child.stderr.take().unwrap();

    let reader = BufReader::new(stderr);

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line)
        }
    }

    let status = child.wait()?;
    if status.success() {
        println!("Download Selesai");
    } else {
        println!("Download Gagal");
    }

    Ok(())
}

pub fn fetch_title(url: &str) -> Result<String, ()> {
    let output = Command::new("yt-dlp")
        .arg("-e") // get title
        .arg(url)
        .output()
        .map_err(|_| ())?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(title)
    } else {
        Err(())
    }
}
