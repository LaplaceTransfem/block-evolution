// example to help get started
fn main(){
	let mut graph:Graph<Layer<NdArray>>=Graph::new();								// create mutable variable storing a new graph of Layer with the NdArray backend. We can use Wgpu later for GPU acceleration, but that's a bit overkill for this example
	let x:Vec<f32>=(0..100).map(|n|(n%10) as f32*0.1).collect::<Vec<_>>();			// this is just some vector of 100 floats. It didn't have to be these floats specifically but it is
	let x=Value::from(x);															// create a Value from the vector. Value stores a tensor of either bool, float, int, or multi, or a error, and may have rank from 1-8

	graph.connect("x","y").with(Layer::linear(true,100,10,1.0));					// connect a node labeled "x" to a node labeled "y" using a linear layer. These labels don't have to correspond to variable names, but they are kind of like variable names for the inside of the graph

	let y=Unvec(&graph).forward(x);													// apply the graph network by reference to x. AI trait contains the forward method, and is implemented for references too. Since graph can either take a hashmap or vector of inputs, letting it directly have single inputs caused type issues, so Unvec is a convenience wrapper for putting the input in a vec and taking the output out of the vec
	println!("{}",y.unwrap_f1());													// unwrap a rank 1 float tensor from y to print it. (i apparently forgot to implement display for Value, when I do we'll be able to print the value directly)
}
use block_graph::{																	// imports from block-graph
	AI,Graph,Unvec,burn::{Layer,Value}
};
use burn::backend::NdArray;															// imports from burn
