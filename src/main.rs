use rand::Rng;

fn main() {
    println!("Hello, world!");
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

