// Aldaron's Device Interface - Storage - Demo
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// lib.rs

#[macro_use]
extern crate adi_storage;

fn main() {
	let storage = storage!();

	println!("storage = {}", storage);
}
