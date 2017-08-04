// lib.rs
// Aldaron's Device Interface / Storage
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[macro_use]
extern crate adi_storage;

fn main() {
	let storage = storage!();

	println!("storage = {}", storage);
}
