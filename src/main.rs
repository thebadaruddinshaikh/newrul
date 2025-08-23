use rand::Rng;
use std::ops::{Add, Mul, Sub, Div};

fn main() {
    let mut rng = rand::thread_rng();

    let input: Vec<f32> = (0..20).map(|_| rng.gen_range(0.0..=1.0)).collect();
    let layer1 = Layer::new(20, 10);
    let layer2 = Layer::new(10, 5);
    let layer3 = Layer::new(5, 1);

    let input1 = layer1.activate(&input);
    let input2 = layer2.activate(&input1);
    let output = layer3.activate(&input2);
    
    println!("{:#?}", output);

}

struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    fn new(nin: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights: Vec<f32> = (0..nin).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let bias: f32 = rng.gen_range(-1.0..1.0);

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

struct Layer {
    neurons: Vec<Neuron>,
}


impl Layer {
    fn new(nin: usize, nout: usize) -> Self {
        let neurons = (0..nout).map(|_| Neuron::new(nin)).collect();
        Self { neurons }
    }

    fn activate(&self, input: &[f32]) -> Vec<f32> {
        self.neurons.iter().map(|n| n.activate(input)).collect()
    }
}

struct Value{
    data: f32,
    grad: f32,
}

impl Value {
    fn new(data: f32) -> Self {
        Self { data, grad: 0.0 }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Value::new(self.data + other.data)
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Value::new(self.data - other.data)
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Value::new(self.data * other.data)
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Value::new(self.data / other.data)
    }
}

// struct NN {
//     layers: Vec<Layer>,
// }

// impl NN {
//     fn new() -> Self {
//         Self { layers: Vec::new() }
//     }

//     fn sequential(layers: Vec<Layer>) -> Self {
//         Self { layers }
//     }

//     fn linear(&self, nin: u32, nout: u32) -> Self {
//         Self { layers: vec![Layer::new(nin as usize, nout as usize)] }
//     }
// }   