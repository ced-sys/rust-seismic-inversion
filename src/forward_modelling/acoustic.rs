//! Acoustic wave equation solver

use ndarray::{Array1, Array2};
use num_complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex64};

///Acoustic forward modelling parameters
#[derive(Debug, Clone)]
pub struct AcousticModel{
    pub velocity: Array2<f64>,
    pub density: Array2<f64>,
    pub dt: f64,
    pub dx: f64,
    pub nt: usize,
    pub nx: usize,
}

impl AcousticModel {
    ///Create a new acoustic model
    pub fn new(nx: usize, nt: usize, dt: f64, dx: f64) -> Self {
        Self {
            velocity: Array2::zeros((nx, nx)),
            density: Array2::ones((nx, nx))*1000.0,
            dt,
            dx,
            nt,
            nx,
        }
    }

    //Set up a simple layered velocity model
    
}