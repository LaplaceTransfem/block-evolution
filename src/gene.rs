// allowed tokens in the "gene" sequence. This list is not final; we might add specific tokens for layer types
pub const ALLOWED_TOKENS:[u32;31]=
[
	'A' as u32,// letters
	'B' as u32,
	'C' as u32,
	'D' as u32,
	'E' as u32,
	'F' as u32,
	'G' as u32,
	'H' as u32,
	'I' as u32,
	'J' as u32,
	'K' as u32,
	'L' as u32,
	'M' as u32,
	'N' as u32,
	'O' as u32,
	'P' as u32,
	'Q' as u32,
	'R' as u32,
	'S' as u32,
	'T' as u32,
	'U' as u32,
	'V' as u32,
	'W' as u32,
	'X' as u32,
	'Y' as u32,
	'Z' as u32,
	' ' as u32,	// space for separator
	';' as u32,	// semicolon for stop codon
	256,		// begin connection
	257,		// begin node
	258,		// begin layer
];
/// at each position in the gene, possibly apply the three types of point mutations according to their respective probabilities
pub fn mutate(mut gene:Vec<u32>,deletionchance:f32,insertionchance:f32,substitutionchance:f32)->Vec<u32>{
	todo!()
}
/// returns true with probability chance
pub fn should_mutate(chance:f32)->bool{
	let choice:f32=rand::random();
	choice<chance
}

/// generate a gene from a graph given a hashmap of 
pub fn gene_from_graph<C: AI<V,V> + Op<Output = V>,V: Clone + Default + Merge>(graph: &Graph<C>, map: Option<HashMap::<u32,Label>>)
->(Vec<u32>,Option<HashMap::<u32,Label>>){
	let mut gene: Vec<u32> = Vec::new();
	let map = match map{ // check if hashmap was input or not 
		Some(HashMap::<u32,Label>) => map,
		None => Some({ // TODO: generate new hashmap that can be returned
			HashMap::<u32,Label>::new()
		})
	}.unwrap();
	for connection in graph.connections(){
		
	}
	(gene,Some(map))
}

pub fn init_random_gene(length:usize)->Vec<u32>{
	let mut gene:Vec<u32>=Vec::with_capacity(length);
	for _ in 0..length{ // not inclusive of length*
		gene.push(ALLOWED_TOKENS[rand::rng().random_range(0..ALLOWED_TOKENS.len()-1)]); 
	}
	gene
}

use {
	block_graph::{Graph,Label,Op,AI,Merge},
	std::{default::Default,collections::HashMap},
	rand::Rng
};