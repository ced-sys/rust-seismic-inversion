//! High-Performance Seismic Inversion Tool
//!
//!A Rust based toool for seismic data processing, forward modelling,
//! and inversion using FFT-based algorithms

mod forward_modelling;
mod inversion;
mod io;
mod utils;

use forward_modelling::run_forward_model;

fn main(){
    println!("Seismic Inversion Tool Initialized");
    println!("Runnig forward seismic model...");

    //Run forward modelling
    match run_forward_model(){
        Ok(_)=>println!("Forward modelling completed successfully"),
        Err(e)=>eprimtlm!("Error in forward modelling:{}", e),
    }
}