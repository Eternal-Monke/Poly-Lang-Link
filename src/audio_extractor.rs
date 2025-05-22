
use std::process::{Command, Stdio};
use std::path::Path;

pub fn extract_audio(input: &str, output: &str) {

    let status = Command::new("ffmpeg").args(
        ["-i", input, "-vn", "-acodec", "-mp3", output,]
    ).stdout(Stdio::null()).stderr(Stdio::null()).status();
    
    match status {
        Ok(out) => {
            if out.success() {
                println!("Audio extracted successfully to '{}'", output);
            } else {
                eprintln!("ffmpeg failed. Make sure it's installed.");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Failed to start ffmpeg error encountered is {}", error);
            std::process::exit(1);
        }
    }


}
