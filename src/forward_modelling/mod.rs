use anyhow::Result;
use crate::convolution::ConvolutionEngine;
use crate::models::ReflectivityModel;
use crate::wavelets::RickerWavelet;

///Seismic forward modelling pipeline
///
/// This orchestrates the complete forward modellin process:
///1. Takes a reflectivity model (Earth structure)
///2. Convolves with a source wavelet
///3. Produces synthetic seismograms
pub struct SeismicPipeline{
    /// FFT-based convolution engine
    convolution_engine: ConvolutionEngine,
    /// Pipeline configuration
    config: PipelinConfig,
}

/// Configuration parameters for the seismic pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig{
    ///Add random noise to the synthetic data
    pub add_noise: bool,
    pub noise_level: f64,
    pub apply_filter: bool,
    pub low_freq: f64,
    pub high_freq: f64,
    pub sample_rate: f64,
}

impl Default for PipelineConfig{
    fn default()->Self{
        Self{
            add_noise: false,
            noise_level: 0.01,
            apply_filter: false,
            low_freq: 5.0,
            high_freq: 100.0
            sample_rate: 1000.0,
        }
    }
}

///Resuts from forward modelling
#[derive(Debug)]
pub struct ForwardModellingResults{
    ///Synthetic seismogram
    pub synthetic_trace: Vec<f64>,
    ///Input reflectivity model
    pub reflectivity: Vec<f64>,
    ///Source wavelet used
    pub wavelet: Vec<f64>,
    ///Time vector for the synthetic trace
    pub time: Vec<f64>,
    /// Processing statistics
    pub stats: ProcessingStats,
}

///Statistics from the forward modelling process
#[derive(Debug)]
pub struct ProcessingStats{
    pub reflectivity_sparsity: f64,
    pub wavelet_dominant_freq: f64,
    pub output_snr: f64,
    pub processing_time_ms: f64,
    pub onvolution_length: usize,
}

impl SeismicPipeline{
    ///Create a new seismic pipeline with defualt configuration
    pub fn new()-> Self{
        Self{
            convolution_engine: ConvolutionEngine::new(),
            config: PipelineConfig::default(),
        }
    }

    ///Create a pipeline with custom configuration
    pub fn with_config(config: PipelineConfig)-> Self{
        Self{
            convolution_engine: ConvolutionEngine::new(),
            config.
        }
    }

    //Run complete forward modelling workflow
    pub fn run_forward_modelling(
        &mut self,
        reflectivity_model: &ReflectivityModel,
        wavelet: &RickerWavelet,
    )-> Result<ForwardModellingResults>{
        let start_time=std::time::Instant::now();

        //Step 1: Convolve reflectivity with wavelet
        let mut synthetic_trace=self.convolution_engine.convolve(
            &refectivity_model.coefficients,
            &wavelet.samples,
        )?;

        //Step 2: Add noise if requested
        if self.config.add_noise{
            self.add_noise_to_trace(&mut synthetic_trace);
        }

        //Step 3: Apply filtering if requested
        is self.config.apply_filter{
            self.apply_bandpass_filter(&mut synthetic_trace)?;
        }

        //Step 4: generate time vector
        let dt=1.0/self.config.sample_rate;
        let time: Vec<f64> =(0..synthetic_trace.len()).map(|i| i as f64 *dt).collect();

        //Step 5: Calculate statistics
        let processing_time=start_time.elapsed();
        let model_stats=reflectivity_model.stats();

        let signal_power: f64=synthetic_trace.iter().map(|x| x*x).sum();
        let noise_power=if self.config.add_noise{
            let noise_var=(self.config.noise_level*self.estimate_signal_len(&synthetic_trace)).powi(2);
            noise_var*synthetic_trace.len() as f64
        }else{
            1e-12 //Very small value for numerical stability
        };
        let snr=10.0* (signal_power/noise_power.max(1e-12)).log10();

        let stats=ProcessingStats{
            reflectivity_sparsity: model_stats.sparsity,
            wavelet_dominant_freq: wavelet.frequency,
            output_snr: snr,
            processing_time_ms: processing_time.as_secs_f64()*1000.0,
            convolution_length: synthetic_trace.len()
        };

        Ok(ForwardModellingResults {
            synthetic_trace,
            reflectivity: reflectivity_model.coefficients.clone(),
            wavelet: wavelet.samples.clone(),
            time,
            stats,
        })
    }

    /// Generate multiple realization with different noise
    pub fn run_monte_carlo(
        &mut self,
        reflectivity_model: &ReflectivityModel,
        wavelet: &RickerWavelet,
        num_realizations: usize,
    )-> Result<Vec<ForwardModellingResults>> {
        let mut results=Vec::with_capacity(num_realizations);
        let original_noise_setting=self.config.add_noise;

        //Ensure noise is enabled for Monte Carlo
        self.config.add_noise=true;

        for i in 0..num_realizations{
            println!("Running realization {}/{}", i+1, num_realizations);
            let result=self.run_forward_modelling(reflectivity_model, wavelet)?;
            results.push(result);
        }

        //Restore original noise setting
        self.config.add_noise=original_noise_setting;

        Ok(results)
    }

    /// Add random noiseto the synthetic trace
    fn add_noise_to_trace(&self, trace: &mut [f64]){
        let signal_level=self.estimate_signal_level(trace);
        let noise_amplitude=self.config.noise_level* signal_level;

        for sample in trace.iter_mut(){
            let noise=noise_amplitude*(2.0*fastrand::f64()-1.0);
            *sample+=noise;
        }
    }

    /// Estimate the signal level for noise scaling
    fn estimate_signal_level(&self, trace: &[f64])-> f64{
        // Use RMS as signal level estimate
        let rms: f64=trace.iter().map(|x| x*x).sum::<f64>()/ trace.len() as f64;
        rms.sqrt()
    }

    ///Apply simple bandpass filter (placeholder)
    fn apply_bandpass_filter(&self, trace: &mut [f64])-> Result<()> {

        println!("Applying bandpass filter: {:.1}-{:.1} Hz",
    self.config.low_freq, self.sonfig.high_freq);

    //Simple moving average as a low-pass filter appr
    let window_size=(self.config.sample_rate/ (2.0 *self.config.high_freq)) as usize;
    if window_size > 1 && window_size<trace.len()/4{
        self.apply_moving_average(trace, window_size);
    }

    Ok(())
    }

    /// Apply moving average fiilter
    fn apply_moving_average(&self, trace: &mut [f64], window_size: usize){
        let mut filtered=vec![0.0; trace.len()];
        let half_window=window_size/2;

        for i in 0..trace.len(){
            let start=i.saturating_sub(half_window);
            let end=(i+ half_window+1).min(trace.len());
            let window_len=end-start;

            let sum: f64=trace[start..end].iter().sum();
            filtered[i]=sum/window_len as f64;
        }

        trace.copy_from_slice(&filtered);
    }

    ///Update pipeline configuration
    pub fn set_config(&mut self, config: PipelineConfig){
        self.config=config;
    }

    //Get current configuration
    pub fn config(&self)-> &PipelineConfig{
        &self.config
    }
}

impl Default for SeismicPipeline{
    fn default()-> Self{
        Self::new()
    }
}

///Batch processing for multiple models
pub struct BatchProcessor{
    pipeline: SeismicPipeline,
}

impl BatchProcessor{
    pub fn new(config; PipelineConfig)-> Self{
        Self{
            pipeline: SeismicPipeline::with_config(config),
        }
    }

    ///Process multiple reflectivity models with same wavelet 
    pub fn process_models(
        &mut self,
        models: &[ReflectivityModel],
        wavelet: &RickerWavelet,
    )-> result<Vec<ForwardModellingResults>> {
        let mut results=Vec::with_capacity(models.len());

        for (i, model) in models.iter().enumerate(){
            println!("Processing model {}/{}", i+1, models.len());
            let result=self.pipeline.run_forward_modelling(model, wavelet)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Process one model with multiple wavelets
    pub fn process_wavelets(
        &mut self,
        model: &ReflectivityModel,
        wavelets: &[RickerWavelet],
    )-> Result<Vec<ForwardModellingResults>> {
        let mut results=Vec::with_capacity(wavelets.len());

        for(i, wavelet) in wavelets.iter().enumerate(){
            println!("Processing wavelet {}/{} ({:.1}) Hz)", i+1, wavelets.len(), wavelet.frequency);
            let result=self.pipeline.run_forward_modelling(model, wavelet)?;
            results.push(result);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_basic_forward_modelling()-> Result<()> {
        let mut pipeline=SeismicPipeline::new();

        let model=ReflectiveModel::new(100, vec![25, 50, 75], vec![0.1, -0.05, 0.15]);
        let wavelet=RickerWavelet::new(30.0, 0.001, 50)?;

        let results=pipeline.run_forward_modelling(&model, &wavelet)?;

        assert_eq!(results.reflectivity.len(), 100);
        assert_eq!(results.wavelet.len(), 50);
        assert_eq!(results.synthetic_trace.len(), 149);
        assert_eq!(results.time.len(), 149);

        Ok(())
    }
    #[test]
    fn test_pipeline_with_noise()-> Result<()>{
        let config=PipelineConfig{
            add_noise: true,
            noise_level: 0.05,
            ..Default::default()
        };

        let mut pipeline=SeismicPipeline::with_config(config);

        let model=ReflectivityModel::new(50, vec![10, 30], vec![0.2, -0.1]);
        let wavelet=RickerWavelet::new(25.0, 0.001, 40)?;

        let results=pipeline.run_forward_modelling(&model, &wavelet)?;

        assert!(results.stats.output_snr>10.0);

        Ok(())
    }

    #[test]
    fn test_monte_carlo()-> Result<()>{
        let mut pipeline=SeismicPipeline::new();

        let model=ReflectivityModel::new(30, ve![10, 20], vec![0.15, -0.1]);
        let wavelet=RickerWavelet::new(40.0, 0.001, 30)?;

        let results=pipeline.run_monte_carlo(&model, &wavelet, 3)?;

        assert_eq!(results.len(), 3);

        //All realizations shoud have the same dimensions
        for result in &results{
            assert_eq!(result.synthetic_trace.len(), 58);
            assert_eq!(result.reflectivity.len(), 30);
        }

        Ok(())
    }
}
