use anyhow::Result;
use num_complex::Complex;
use rustfft::{FftPlanner, Fft};
use std::sync::Arc;

/// High performance FFT-based convolution engine for seismic processing
pub struct ConvolutionEngine{
    planner::FftPlanner<f64>,
}

impl ConvolutionEngine{
    ///Create a new convolution engine
    pub fn new()-> Self {
        Self{
            planner: FftPlanner::new(),
        }
    }

    ///Compute convolutin of two real-valued signals using FFT
    ///
    /// This is the core operation for seismic forward modelling:
    /// Synthetic_trace=reflectivity & wavelet
    pub fn convolve(&mut self, signal_a: &[f64], signal_b: &[f64])-> Result<Vec<f64>> {
        if signal_a.is_empty() || signal_b.is_empty(){
            return Ok(vec![]);
        }

        //Calculate output length for linear convolution
        let output_len=signal_a.len()+signal_b.len()-1;

        //Find next power of 2 for efficient FFT
        let fft_len=next_power_of_2(output_len);

        printn!("Convolution details:");
        println!("Signal A lenght: {} samples", signal_a.len());
        println!("Signal B length: {} samples", signal_b.len());
        println!("Output length: {} samples", output_len);
        println!("FFT length: {} samples (padded)", fft_len);

        //Create FFT and IFFT plans
        let fft=self.planner.plan_fft_forward(fft_len);
        let ifft=self.prepare_fft_buffer(signal_b, fft_len);

        //Forward FFT
        fft.process(&mut buffer_a);
        fft.process(&mut buffer_b);

        //Frequency domain multiplication (convolution theorem)
        let mut result_buffer: Vec<Complex<f64>> = buffer_a.iter().zip(buffer_b.iter()).map(|a(a, b)| a*b).collect();

        //Inverse FFT
        ifft.process(&mut result_buffer);

        //Extract real part and normalize
        let normalization_factor=1.0/fft_len as f64;
        let result: Vec<f64>=result_buffer.iter().take(output_len).map(|c| c.re*normalization_factor).collect();

        Ok(result)

    }

    /// Prepare a real signal for FFT processing
    fn prepare_fft_buffer(&self, signal: &[f64], fft_len: usize)-> Vec<Complex<f64>> {
        let mut buffer=Vec::with_capacity(fft_len);

        //Copy signal data
        for &sample in signal{
            buffer.push(Complex::new(sample, 0.0));
        }

        //Zero-pad to FFT length
        buffer.resize(fft_len, Complex::new(0.0, 0.0));

        buffer
    }

    //Compute cross-correlation using fft (for future use in inversion)
    pub fn cross_correlate(&mut self, signal_a: &[f64], signal_b: &[f64])-> Result<Vec<f64>>{
        if signal_a.is_empty()|| signal_b.is_empty(){
            return Ok(vec![]);
        }

        let output_len=signal_a.len()+signal_b.len()-1;
        let fft_len=next_power_of_2(output_len);

        let fft=self.planner.plan_fft_forward(fft_len);
        let ifft=self.planner.plan_fft_inverse(fft_len);

        let mut buffer_a=self.prepare_fft_buffer(signal_a, fft_len);
        let mut buffer_b=self.prepare_fft_buffer(signal_b, fft_len);

        fft.process(&mut buffer_a);
        fft.process(&mut buffer_b);

        //Cross-correlation in frequency domian: A* B=FFT^-1 (A* xB)
        let mut result_buffer: Vec<Complex<f64>>=buffer_a.iter().zip(buffer_b.iter()).map(|(a, b)| a.conj()*b).collect();

        ifft.process(&mut result_buffer);

        let normalization_factor=1.0/fft_len as f64;
        let result: Vec<f64>=result_buffer.iter().take(output_len).map(|c| c.re*normalization_factor).collect();

        Ok (result)
    }

    ///Auto-correlation (useful for wavelet analysis)
    pub fn auto_correlate(&mut self, signal:&[f64])-> Result<Vec<f64>> {
        self.cross_correlate(signal, signal)
    }
}

impl Default for ConvolutionEngine{
    fn default()-> Self{
        Self::new()
    }
}

///Find the next power of 2 greater than or equal to n
fn next_power_of_2(n: usize)-> usize{
    if n<=1{
        return 1;
    }

    let mut power=1;
    while power < n{
        power<=1;
    }
    power
}

#[cfg(test)]
mod tests{
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_next_power_of_2(){
        assert_eq!(next_power_of_2(1), 1);
        assert_eq!(next_power_of_2(2), 2);
        assert_eq!(next_power_of_2(3), 4);
        assert_eq!(next_power_of_2(100), 128);
        assert_eq!(next_power_of_2(256), 256);
        assert_eq!(next_power_of_2(257), 512);
    }

    #[test]
    fn test_simple_convolution()-> Result<()>{
        let mut engine=ConvolutionEngine::new();

        //Simple test: Convolve [1, 0, 0] with [1, 2, 3]
        let signal_a=vec![1.0, 0.0, 0.0];
        let signal_b=vec![1.0, 2.0, 3.0];

        let result=engine.convolve(&signal_a, &signal_b)?;
        let expected=vec![1.0, 2.0, 3.0, 0.0, 0.0];

        assert_eq!(result.len(), expected.len());
        for(i, (&actual, &expected)) in result.iter().zip(expected.iter()).enumerate(){
            assert_abs_diff_eq!(actual, expected, epsilon=1e-10,"Mismatch at index {}: {} vs {}", i, actual, expected);
        }

        Ok(())
    }

    #[test]
    fn test_empty_signals()-> Result<()> {
        let mut engine=ConvolutionEngine::new();

        let result=engine.convolve(&[], &[1.0, 2.0])?;
        assert!(result.is_empty());

        let result=engine.convolve(&[1.0, 2.0], &[])?;
        assert!(result.is_empty());

        Ok(())
    }
    #[test]
    fn test_symmetry()-> Result<()> {
        let mut engine=ConvolutionEngine::new();

        let signal_a=vec![1.0, 2.0, 3.0];
        let signal_b=vec![4.0, 5.0];

        let result_ab=engine.convolve(&signal_a, & signal_b)?;
        let result_ba=engine.convolve(&signal_b, &signal_a)?;

        assert_eq!(result_ab.len(), result_ba.len());
        for(i, (&a, &b)) in result_ab.iter().zip(result_ba.iter()).enumerate(){
            assert_abs_diff_eq!(a,b, epsilon=1e-10, "Symmetry failed at index {}: {} vs {}", i, a ,b);
        }

        Ok(())
    }
}