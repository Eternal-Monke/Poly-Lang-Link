
use std::process::{Command, Stdio};

pub fn extract_audio(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {

    let status = Command::new("ffmpeg").args(
        ["-i", input,"-vn", "-ac", "1", "-ar", "16000", "-sample_fmt", "s16", output]
    ).stdout(Stdio::null()).stderr(Stdio::null()).status();
    
    match status {
        Ok(out) => {
            if out.success() {
                println!("Audio extracted successfully to '{}'", output);
                Ok(())
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
