use anyhow::{Result, Context};
use csv::Writer;
use std::fs::File;
use std::io::Write;

///Export data to CSV file
pub fn export_to_csv(data: &[f64], filename: &str)-> Result<()>{
    let file=File::create(filename).with_context(|| format!("Failed to create file: {}", filename))?;

    let mut writer=Writer::from_writer(file);

    //Write header
    writer.write_record(&["sample", "amplitude"])?;

    //Write data
    for (i, &value) in data.iter().enumerate(){
        writer.write_record(&[i.to_string(), value.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}

/// Simple ASCII plotting for terminal visualzation
pub fn plot_ascii(data: &[f64], height:usize){
    if data.is_empty(){
        println!("(No data to plot)");
        return;
    }

    //Find data range
    let min_val=data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val=data.iter()
}
