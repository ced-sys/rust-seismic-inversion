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
    let max_val=data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if min_val==max_val{
        println!("(Constant value:{:.6})", min_val);
        return;
    }

    let range=max_val-min_val;

    //Create plot lines
    for row in (0..height).rev(){
        let threshold=min_val+(row as f64+0.5)*range/height as f64;
        print!(" ");

        for &value in data{
            if value >= threshold{
                print!("|");
            }else if value>=threshold-range/(2.0* height as f64){
                print!("/|");
            }else{
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

/// Enhanced ASCII plotting with axis labels
pub fn plot_ascii_with_axis(
    data: &[f64],
    height: usize,
    width: Option<usize>,
    title: &str,
){
    println!(" {}", title);

    if data.is_empty(){
        println!( "(No data to plot)");
        return;
    }

    let plot_width=width.unwrap_or(data.len().min(80));
    let min_val=data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val=data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if min_val==max_val{
        println!(" (Constanct value: {:.6})", min_val);
        return;
    }

    let range=max_val-min_val;

    // Downsampl data if needed
    let plot_data=if data.len()>plot_width{
        downsample_data(data, plot_width)
    }else{
        data.to_vec()
    };

    //Plot
    for row in (0..height).rev(){
        let threshold=min_val+(row as f64+0.5)*range/height as f64;
        print!(" ");

        //Y-axis label
        if row==height-1{
            print!("{:>8.3}|", max_val);
        }else if row==0{
            print!("{:>8.3}|", min_val);
        }else if row==height/2{
            print!("{:>8.3}|", (max_val+min_val)/2.0);
        }else{
            print!("        |");
        }   
    }
}
