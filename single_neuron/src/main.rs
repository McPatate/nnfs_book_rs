fn neuron<'a>(inputs: &'a Vec<f64>, weights: &'a Vec<f64>, bias: f64) -> f64 {
    let mut result = bias;
    for cursor in inputs.iter().zip(weights.iter()) {
        let (i, w) = cursor;
        result += i * w;
    }
    result
}

fn main() {
    let inputs: Vec<f64> = vec![1., 2., 3., 2.5];
    let weights: Vec<f64> = vec![0.2, 0.8, -0.5, 1.];
    println!("{}", neuron(&inputs, &weights, 2.0));
}
