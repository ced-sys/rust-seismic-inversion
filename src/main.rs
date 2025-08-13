use anyhow::Result;
use std::time::Instant;

mod convolution;
mod forward_modelling;
mod models;
mod utils;
mod wavelets;

use convolution::ConvolutionEngine;
use forward_modelling::SeismicPipeline;
use models::ReflectivityModel;
use utils::{export_to_csv, plot_ascii, Statistics};
use wavelets::RickerWavelet;

fn main()->Result<()> {
    println!("Rust Seismic Inversion Tool Starting...\n");

    let start_time=Instant::now();

    //Step 1: Create a reflectivity model
    println!("Defining reflectivity model...");
    let reflectivity_model=ReflectivityModel::new(100, vec![20, 40, 60, 80], vec![0.1, -0.05, 0.15, -0.08]);
    println!("Model length: {} samples", reflectivity_model.coefficients.len());
    println!("Reflectivity coefficients: {:?}\n", reflectivity_model.reflection_coefficients);

    //Step 2: Generate a Ricker wavelet
    println!("Generating a Ricker wavelet...");
    let wavelet=RickerWavelet::new(30.0, 0.001, 200)?;
    println!("Dominant frequency: {} Hz", wavelet.frequency);
    println!("Sample rate: {} s", wavelet.dt);
    println!("Wavelet length: {} samples\n", wavelet.samples.len());

    //Step 3: Setup convolution engine
    println!("Step 3: Setting up FFT-based convolution engine...");
    let mut conv_engine=ConvolutionEngine::new();

    let input_len=reflectivity_model.coefficients.len()+wavelet.samples.len()-1;
    println!("Expected output length: {} samples", input_len);

    //Perform convolution
    let synthetic_trace=conv_engine.convolve(&reflectivity_model.coefficients, &wavelet.sample)?;
    println!("Convolution completed");
    println!("Actual output length: {} samples\n", synthetic_trace.len());

    //Step 4: Run forward modelling pipeline
    println!("Stop 4: Running forward modelling pipeline...");
    let mut pipeline=SeismicPipeline::new();
    let results=pipelinerun_forward_modelling(&reflectivity_model, &wavelet)?;

    //Calculate statistics
    let trace_stats=Statistics::calculate(&synthetic_trace);
    let energy_ratio=trace_stats.energy / wavelet.samples.iter().map(|x| x*x).sum::<f64>();

    println!("Syynthetic trace statistics:");
    println!("Min amplitude: {:.6}", trace_stats.min);
    println!("Max amplitude: {:.6}", trace_stats.max);
    println!("Mean amplitude: {:.6}", trace_stats.mean);
    println!("Std deviation: {:.6}", trace_stats.std_dev);
    println!("RMS amplitude:{:.6}", trace_stats.rms);
    println!("Total energy:{:.6}", trace_stats.energy);
    println!("Energy ratio:{:.3}\n", energy_ratio);

    //Step 5: Export results
    println!("Step 5: Exporting results...");
    export_to_csv(&synthetic_trace, "synthetic_trace.csv")?;
    println!("Exported {} samples to synthetic_trace.csv", synthetic_trace.len());

    export_to_csv(&reflectivity_model.coefficients, "reflectivity_model.csv")?;
    println!("Exported {} samples to reflectivity_model.csv", relfectivity_model.coefficients.len());

    export_to_csv(&wavelet.samples, "ricker_wavelet.csv")?;
    println!("Exported {} samples to ricker_wavelet.csv\n", wavelet.samples.len());

    // Step 6: ASCII visualization
    println!("ASCII visualization preview...");
    println!("\nSynthetic Seismogram (First 50 samples):");
    plot_ascii(&synthetic_trace[..50.min(synthetic_trace.len())], 20);

    println!("\nRicker wavelet (ceter portion):");
    let wavelet_center_start=(wavelet.samples.len()/2).saturating_sub(25);
    let wavelet_center_end=(wavelet_center_start+50).min(wavelet.samples.len());
    plot_ascii(&wavelet.samples[wavelet_center_start..wavelet_center_end], 20);

    //Final summary
    let elapsed=start_time.elapsed();
    println!("\nSeismic forward modelling completed successfully!");
    println!("Total execution time: {:.3}ms", elapsed.as_secs_f64()*1000.0);
    println!("Performance: {:.0} samples/ms", synthetic_trace.len() as f64 / (elapsed.as_secs_f64()*1000.0));

    Ok(())

}