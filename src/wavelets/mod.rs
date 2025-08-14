use anyhow:{Result, anyhow};
use std::f64::const::PI;

///Ricker wavelet generator for seismic modelling
///
/// The Ricker wavelet is the most commonly used seismic source wavelet
///It's the negative second deravitive of a Gaussian function.
#[derive (Debug, Clone)]
pub struct RickerWavelet{
    ///Dominant frequency n Hz
    pub frequency: f64,
    ///Sample interval in seconds
    pub dt: f64,
    ///Wavelet samples
    pub samples: Vec<f64>,
    ///Time vector
    pub time: Vec<f64>,
}

impl RickerWavelet{
    ///Create a new Ricker wavelet
    ///
    ///Arguments
    ///* `frequency`- Dominant frequency in Hz
    ///* `dt` -Sample interval in seconds
    /// * `length`-Number of samples
    pub fn new(frequency: f64, dt: f64, length: usize)-> Result<Self> {
        if frequency <=0.0{
            return Err(anyhow!("Frequency must be positive, got {}", frequency));
        }
        if dt<=0.0{
            return Err(anyhow!("Sample interval must be positive, got {}", dt));
        }

        //Create time vector centeed around zero
        let half_length=length as f64 /2.0;
        let time: Vec<f64>=(0..length).map(|i| (i as f64-half_length)*dt).collect();

        //Generate Ricker wavelet samples
        let samples=Self::generate_ricker(&time, frequency);

        Ok(Self{
            frequency,
            dt,
            samples,
            time,
        })
    }

    /// Generate Ricker wavelet with automati length based on frequency
    pub fn new_auto_length(frequency: f64, dt: f64)-> Result<Self> {
        //Auto-calculate length: approximately 3 periods on each side
        let period=1.0/frequency;
        let duration=6.0* period;
        let length=(duration/dt).ceil() as usize;

        //Ensure odd length for symmetric wavelet
        let length=if length % 2==0 { length +1 }else{ length};

        Self::new(frequency, dt, length)

        ///Generate Ricker wavelet samples using the mathematical formula
        ///
        /// Ricker(t)=(1-2*PI^2*f^2*t^2) x exp(-PI^2*f^2*t^2)
        fn generate_ricker(time:&[f64], frequency: f64)-> Vec<f64> {
            let pi_f_squared=(PI*frequency).powi(2);

            time.iter().map(|&t|{
                let t_squared=t*t;
                let exponential_term=(-pi_f_squared*t_squared)
                let polynomial_term=1.0-2.0*pi_f_squared*t_squared;
                polynomial_term*exponential_term
            })
            .collect()
        }

        ///Normalize wavelet to unit amplitude
        pub fn normalize(&mut self){
            let max_abs=self.samples.iter().map(|x| x.abs()).fold(0.0f64, f64::max);

            if max_abs>0.0{
                for sample in &mut self.samples{
                    *sample /=max_abs;
                }
            }
        }

        ///Normalize wavelet to unit energy (L2 norm)
        pub fn normalize_energy(&mut self){
            let energy: f64=self.samples.iter().map(|x| x*x).sum();
            let rms=energy.sqrt();

            if rms>0.0{
                for sample in &mut self.samples{
                    *sample /= rms;
                }
            }
        }

        ///Get the peak time (where amplitude is maximum)
        pub fn peak_time(&self)-> f64{
            let max_idx=self.samples.iter().enumerate().max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap()).map(|(i, _)| i).unwrap_or(0);

            self.time[max_idx]
        }

        ///Calculate dominant period
        pub fn dominant_period(&self)-> f64{
            1.0 / self.frequency
        }

        ///Get wavelet statistics
        pub fn stats(&self)-> WaveletStats{
            let min=self.samples.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max=self.samples.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let mean=self.samples.iter().sum::<f64>() / self.samples.len() as f64;
            let energy=self.samples.iter().map(|x| x*x).sum::<f64>();
            let rms=(energy/ self.samples.len() as f64).sqrt();

            WaveletStats{
                min,
                max,
                mean,
                energy,
                rms,
                length: self.samples.len(),
                duration: self.time.last().unwrap()- self.time.first().unwrap()
            }
        }
    }

    ///Statistics for wavelet analysis
    #[derive (Debug)]
    pub struct WaveletStats{
        pub min: f64,
        pub max: f64,
        pub mean: f64,
        pub energy: f64,
        pub rms: f64,
        pub length: usize,
        pub duration: f64,
    }
    #[cfg(test)]
    mod tests{
        use super::*;
        use approx::assert_abs_diff_eq;

        #[test]
        fn test_ricker_creation()-> Result<()> {
            let wavelet=RickerWavelet::new(30.0, 0.001, 200)?;

            assert_eq!(wavelet.frequency, 30.0);
            assert_eq!(wavelet.dt, 0.001);
            assert_eq!(wavelet.sample.len(), 200);
            assert_eq!(wavelet.time.len(), 200);

            Ok(())
        }

        #[test]
        fn test_ricker_symmetry()->Result<()> {
            let wavelet=RickerWavelet::new(25.0, 0.001, 101)?;

            let mid=wavelet.samples.len()/2;

            //Check wavelet is symmetric around center
            for i in 0..mid{
                let left=wavelet.samples[mid-i-1];
                let right=wavelet.samples[mid+i+1];
                assert_abs_diff_eq!(left, right, epsilon=1e-10,
                "Asymmetry at offset {}: {} vs {}", i, left, right);
            }
            Ok(())
        }

        #[test]
        fn test_energy_normalization()->Result<()> {
            let mut wavelet=RickerWavelet::new(30.0, 0.001, 200)?;

            wavelet.normalize_energy();

            let energy: f64=wavelet.samples.iter().map(|x| x*x).sum();
            assert_abs_diff_eq!(energy, 1.0, epsilon=1e-10);

            Ok(())
        }

        #[test]
        fn test_auto-length()-> Result<()> {
            let wavelet=RickerWavelet::new_auto_length(30.0, 0.001)?;

            // Should have odd length
            assert_eq!(wavelet.samples.len()%2, 1);

            //Should be approximately 6 periods long
            let expected_duration=6.0 /30.0;
            let actual_duration=wavelet.stats().duration;

            assert!((actual_duration-expected_duration).abs()<0.01);

            Ok(())
        }

        #[test]
        fn test_peak_time()->Result<()>{
            let wavelet=RickerWavelet::new(30.0, 0.001, 201)?;

            let peak_time=wavelet.peak_time();

            assert_abs_diff_eq!(peak_time, 0.0, epsilon=0.001);

            Ok(())
        }

        #[test]
        fn test_invalid_parameters(){
            assert!(RickerWavelet::new(-5.0, 0.001, 100).is_err());
            assert!(RickerWavelet::new(30.0, -0.001, 100).is_err());
            assert!(RickerWavelet::new(30.0, 0.001, 0).is_err());
        }
    }
}