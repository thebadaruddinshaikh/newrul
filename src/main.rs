fn main() {
    println!("Hello, world!");
}


struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    fn new(weights: Vec<f32>, bias: f32) -> Self {
        Self { weights, bias }
    }

    fn activate(&self, inputs: &[f32]) -> f32 {
         let mut out = self.bias;

         for (weight, input) in self.weights.iter().zip(inputs.iter()) {
            out += weight * input;
        }
        out
    }  
}

