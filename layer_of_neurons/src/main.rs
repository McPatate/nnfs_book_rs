fn neuron(inputs: &[f64], weights: &[f64], bias: f64) -> f64 {
    let mut result = bias;
    for (i, w) in inputs.iter().zip(weights.iter()) {
        result += i * w;
    }
    result
}

fn layer_calc(inputs: &[f64], weights: &[&[f64]], biases: &[f64]) -> Vec<f64> {
    let mut output = Vec::<f64>::with_capacity(biases.len());
    for (neuron_weights, neuron_bias) in weights.iter().zip(biases.iter()) {
        output.push(neuron(inputs, neuron_weights, *neuron_bias));
    }
    output
}

fn main() {
    let inputs: Vec<f64> = vec![1., 2., 3., 2.5];

    let weights: Vec<&[f64]> = vec![
        &[0.2, 0.8, -0.5, 1.],
        &[0.5, -0.91, 0.26, -0.5],
        &[-0.26, -0.27, 0.17, 0.87],
    ];

    let biases: Vec<f64> = vec![2.0, 3.0, 0.5];

    println!("{:?}", layer_calc(&inputs, &weights, &biases));
}
