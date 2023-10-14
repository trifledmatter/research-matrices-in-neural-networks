## Pre-Calculus Matrices Project: Implementing a Neural Network in Rust

### Introduction

> This report presents a project focused on implementing a simple matrices-based neural network in Rust, with an emphasis on integrating pre-calculus concepts related to matrices. The objective was to construct a neural network, utilize appropriate activation functions, and illustrate the application of matrices in machine learning.

#### Key Code Snippets

##### Initializing the Neural Network

```rust
let mut network = Network::new(vec![2, 3, 1], 0.5, SIGMOID);
```

In this line, the neural network is set up with a specific structure (2 input neurons, 3 neurons in the hidden layer, and 1 output neuron), a learning rate of 0.5, and the sigmoid activation function.

##### Forward Propagation

```rust
let outputs = self.feed_forward(inputs[j].clone());
```

This line calls the `feed_forward` function, which computes the neural network's output based on the given input.

##### Backpropagation

```rust
self.back_propagate(outputs, targets[j].clone());
```

Here, the `back_propagate` function is invoked to adjust weights and biases through backpropagation, improving the network's performance.

##### Saving and Loading Network Weights

```rust
self.save(file_path); // Save the weights and biases to a file
self.load(file_path); // Load the weights and biases from a file
```

These snippets demonstrate the ability to save and load the network's weights and biases, enhancing reusability and persistence.

### Mathematical Foundations: Matrices and Pre-Calculus Concepts

> The project leans heavily on matrices for crucial operations like matrix multiplication, addition, and transposition. Key pre-calculus concepts, such as linear algebra and calculus, underpin these operations, ensuring efficient implementation.

#### Matrices in Neural Networks

> - Matrices serve as representations for weights, biases, inputs, and outputs in the neural network.
> - Matrix operations, like multiplication and addition, are fundamental for both forward and backward propagation.

#### Activation Functions

> - Activation functions, such as the sigmoid function, introduce non-linearity by applying them to the weighted sum of inputs.

#### Backpropagation Algorithm

> - Backpropagation leverages calculus concepts, such as the chain rule and partial derivatives, for computing gradients and adjusting weights and biases.

## Why?
> The reason behind this project is that I wanted to bridge the knowledge gap between pre-calculus matrix operations, and neural networks. The motivation stems from a desire to explore and comprehend the underlying connections and applications of pre-calculus within the domain of machine learning.

> The motivation for this project also stems from a personal drive to gain a deeper understanding of the critical role matrices play in the functioning of neural networks. The primary objective is to unravel the intricacies of matrix operations within the context of neural network architecture and training processes. By immersing myself in this implementation, I seek to grasp the underlying principles and mechanisms that facilitate efficient information processing, weight adjustments, and output predictions within a neural network.