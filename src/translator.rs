use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn translate_audio(model_path: &str, audio_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let context_params = WhisperContextParameters::default();
    let context = WhisperContext::new_with_params(model_path, context_params)?;

    let mut state = context.create_state()?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_translate(true);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(true);
    params.set_language(Some("auto"));

    let audio_data = load_audio_file(audio_path)?;

    state.full(params, &audio_data)?;

    let mut transcript = String::new();
    let num_segments = state.full_n_segments()?;

    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        transcript.push_str(&segment);
        transcript.push('\n');
    }

    Ok(transcript)
}

fn load_audio_file(path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(path)?;
    
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect();

    let mut audio = vec![0.0f32; samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut audio)?;

    Ok(audio)
}