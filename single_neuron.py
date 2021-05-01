def neuron(inputs, weights, bias) -> int:
    result = bias
    for i in range(0, len(inputs)):
        result += inputs[i] * weights[i]
    return result

inputs = [1, 2, 3, 2.5]
weights = [.2, .8, -.5, 1.0]
bias = 2

print(neuron(inputs, weights, bias))

