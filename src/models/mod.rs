use anyhow::{Result, anyhow};

///Reflectivity model representing geological layers
///
///This represents the Earth's subsurface as a series of acoustic
/// contrasts that create seismic reflections
#[derive(Debug, Clone)]
pub struct ReflectivityModel{
    ///Reflectivity coefficient values
    pub coefficients: Vec<f64>,
    ///Positions of geological layers (sample indices)
    pub layer_positions: Vec<usize>,
    /// Reflection coefficients for each layer
    pub reflection_coefficients: Vec<f64>,
    ///Total model length in samples
    pub length: usize,
}

impl ReflectivityModel {
    ///Create a new reflectivity model with specified layers
    ///
    ///Arguments
    /// * length -Total length of the model in samples
    /// * layer_positions-Sample positions where reflections occur
    /// *reflection_coefficients-Reflection strength at each position
    pub fn new(
        length: usize,
        layer_positions: Vec<usize>,
        reflection_coefficients: Vec<f64>,
    )-> Self{
        if layer_positions.len()!= reflection_coefficients.len(){
            panic!(
                "Layer positions ({}) and coefficients ({}) must have same length",
                layer_positions.len(),
                reflection_coefficients.len()
            );
        }

        //Initialize coefficients array with zeros
        let mut coefficients=vec![0.0; length];

        //Place reflection coefficients at specified positions
        for (&position, &coefficient) in layer_positions.iter().zip(reflection_coefficients.iter()){
            if position < length{
                coefficients[position]=coefficient;
            }
        }

        Self{
            coefficients,
            layer_positions: layer_postions.clone(),
            reflection_coefficients: reflection_coefficients.clone(),
            length,
        }
    }

    ///Create a simple layered model with evely spaced reflectors
    pub fn new_layered(length: usize, num_layers: usize, layer_spacing: usize)-> Self{
        let layer_positions: Vec<usize> =(1..=num_layers).map(|i|i*layer_spacing).filter(|&pos| pos<length).collect();

        //Generate alternating positive/negative coefficients
        let reflection_coefficients: Vec<f64>=layer_positions.iter().map(|_| {
            let coeff=max_coefficient*(2.0*fastrand::f64()-1.0);
            coeff
        }).collect();

        //Generate alternating positive/negative coefficients
        let reflection_coefficients: Vec<f64>=layer_positions.iter().enumerate().map(|(i, _)| {
            let base_coeff=0.1;
            if i%2==0 {base_coeff} else{-base_coeff}
        }).collect();

        Self::new(length, layer_positions, reflection_coefficients)
    }

    ///Create a wedge model (increasing layer thickness)
    pub fn new_wedge(length: usize, num_layers: usize, initial_spacing: usize)-> Result<Self> {
        if num_layers==0{
            return Err(anyhow!("Number of layers must be positive"));
        }

        let mut layer_positions=Vec::new();
        let mut position=initial_spacing;

        for i in 0..num_layers{
            if position >= length{
                break;
            }
            layer_positions.push(position);
            //Increase spacing for wedge effect
            position += initial_spacing+i*(initial_spacing/4);
        }

        let reflection_coefficients: vec<f64>=layer_positions.iter().enumerate().map
    }
}