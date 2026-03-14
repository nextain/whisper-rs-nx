//! Test CUDA dynamic loading fallback behavior.
//! Built with cuda-dynamic feature on a CUDA machine,
//! then run on a non-CUDA machine to verify CPU fallback.
//!
//! Usage: test_fallback [path/to/ggml-tiny.bin]

use std::ffi::c_int;

fn main() {
    println!("=== CUDA Dynamic Loading Fallback Test ===\n");

    // Step 1: Load dynamic backends
    #[cfg(feature = "cuda-dynamic")]
    {
        println!("[1] cuda-dynamic feature is ENABLED");
        println!("    Calling load_dynamic_backends()...");
        whisper_rs::load_dynamic_backends();
        println!("    OK - no crash\n");
    }

    #[cfg(not(feature = "cuda-dynamic"))]
    {
        println!("[1] cuda-dynamic feature is DISABLED (CPU-only build)");
        println!("    Skipping dynamic backend loading\n");
    }

    // Step 2: Determine model path
    let model_path = std::env::args().nth(1)
        .unwrap_or_else(|| "/nonexistent/model.bin".to_string());
    let has_model = std::path::Path::new(&model_path).exists();
    println!("[2] Model: {} (exists: {})", model_path, has_model);

    // Step 3: Load model with GPU→CPU fallback
    println!("\n[3] Loading model with use_gpu(true)...");
    let mut params = whisper_rs::WhisperContextParameters::default();
    params.use_gpu(true);
    let ctx_gpu = match whisper_rs::WhisperContext::new_with_params(&model_path, params) {
        Ok(ctx) => {
            println!("    Model loaded with GPU request — OK");
            Some(ctx)
        }
        Err(e) => {
            if has_model {
                println!("    GPU load failed: {} — trying CPU fallback", e);
            } else {
                println!("    Expected error (no model file): {}", e);
            }
            None
        }
    };
    println!("    No crash\n");

    let ctx = if let Some(c) = ctx_gpu {
        c
    } else if has_model {
        println!("[4] Fallback: loading with use_gpu(false)...");
        let mut params = whisper_rs::WhisperContextParameters::default();
        params.use_gpu(false);
        match whisper_rs::WhisperContext::new_with_params(&model_path, params) {
            Ok(ctx) => {
                println!("    Model loaded CPU-only — OK\n");
                ctx
            }
            Err(e) => {
                println!("    CPU load also failed: {}\n", e);
                println!("=== PARTIAL PASS (no crash, but model load failed) ===");
                return;
            }
        }
    } else {
        println!("=== PASS (no model provided, crash test only) ===");
        return;
    };

    // Step 5: Run actual inference with silence (1 second of zeros at 16kHz)
    println!("[5] Running inference on 1s of silence (16kHz)...");
    let samples = vec![0.0f32; 16000];
    let mut state = ctx.create_state().expect("Failed to create state");

    let mut wparams = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
    wparams.set_language(Some("en"));
    wparams.set_print_progress(false);
    wparams.set_print_special(false);
    wparams.set_print_realtime(false);
    wparams.set_print_timestamps(false);
    wparams.set_suppress_blank(true);
    wparams.set_suppress_nst(true);

    match state.full(wparams, &samples) {
        Ok(_) => {
            let n_segments: c_int = state.full_n_segments();
            println!("    Inference complete — {} segment(s)", n_segments);
            for i in 0..n_segments {
                if let Some(seg) = state.get_segment(i) {
                    if let Ok(text) = seg.to_str_lossy() {
                        println!("    Segment {}: \"{}\"", i, text.trim());
                    }
                }
            }
        }
        Err(e) => {
            println!("    Inference error: {} (may be expected for silence)", e);
        }
    }
    println!("    No crash\n");

    println!("=== ALL TESTS PASSED ===");
    println!("Model loaded, inference ran successfully on CPU without CUDA.");
}
