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

pub fn gene_from_graph<C: AI<V, V> + Op<Output = V>>(graph: &Graph<C>, map: Option<HashMap::<u32,Label>>)->Vec<u32>{
	let mut gene: Vec<u32> = Vec::new();
	let map = match map{
		Some(HashMap) => map,
		None => Some(HashMap::<u32,Label>::new()) // create hashmap idk
	}.unwrap();
	for connection in graph.connections(){
		
	}
	gene
}

pub fn init_random_gene(length:usize)->Vec<u32>{
	let mut gene:Vec<u32>=Vec::with_capacity(length);
	for _ in 0..length{
		gene.push(ALLOWED_TOKENS[rand::rng::random::<usize>()%ALLOWED_TOKENS.len()]);
	}
	gene
}

use {
	block_graph::{Graph,Label,Op,AI},
	std::{collections::HashMap},
	rand::prelude::*
};