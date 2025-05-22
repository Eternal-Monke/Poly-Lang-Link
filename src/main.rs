
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_video> <output_audio>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    if !Path::new(input).exists() {
        eprintln!("Error: input file '{}' does not exist.", input);
        std::process::exit(1);
    }

    let status = Command::new("ffmpeg").args(["-i", input, "-q:a", "0", "-map", "a", output,]).status();
    
    match status {
        Ok(_) => {},
        Err(error) => {
            println!("Failed to start ffmpeg error encountered is {}", error);
            std::process::exit(1);
        }
    }


}
