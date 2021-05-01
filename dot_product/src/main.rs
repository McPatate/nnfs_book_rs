fn neuron(inputs: &[f64], weights: &[f64], bias: f64) -> f64 {
    inputs
        .iter()
        .zip(weights.iter())
        .map(|(x, y)| x * y)
        .sum::<f64>()
        + bias
}

fn main() {
    let inputs: Vec<f64> = vec![1., 2., 3., 2.5];

    let weights: Vec<&[f64]> = vec![
        &[0.2, 0.8, -0.5, 1.],
        &[0.5, -0.91, 0.26, -0.5],
        &[-0.26, -0.27, 0.17, 0.87],
    ];

    let biases: Vec<f64> = vec![2.0, 3.0, 0.5];

    println!("{}", neuron(&inputs, weights[0], biases[0]));
}
