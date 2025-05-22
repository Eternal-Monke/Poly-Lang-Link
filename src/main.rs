
mod audio_extractor;
mod translator;

use std::env;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_video> <output_audio>", args[0]);
        std::process::exit(1);
    }

    let og_video_path = &args[1];
    let audio_path = &args[2];
    let model_path = "models/ggml-base.bin";

    if !Path::new(og_video_path).exists() {
        eprintln!("Input file '{}' does not exist.", og_video_path);
        std::process::exit(1);
    }

    match audio_extractor::extract_audio(og_video_path, audio_path)?;

    let translated = translator::translate_audio(model_path, audio_path)?;

    let translated_path = Path::new(audio_path).with_extension("txt");
    std::fs::write(&translated_path, &translated)?;

    println!("Transcript saved to {}", translated_path.display());

}
