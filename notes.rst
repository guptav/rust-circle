=============
Learning Rust
=============


Day 0 : Hello-Word
====================
- Installation

Cargo::

	[guptav-virtual-machine 12] > cargo --version
	cargo 1.34.0 (6789d8a0a 2019-04-01)
	
	[guptav-virtual-machine 13] > cargo new hello
	     Created binary (application) `hello` package
	
	[guptav-virtual-machine 14] > cd hello
	
	[guptav-virtual-machine 15] > tree
	.
	├── [guptav           130]  Cargo.toml
	└── [guptav          4096]  src
	    └── [guptav            45]  main.rs

	1 directory, 2 files
	
	[guptav-virtual-machine 17] > cat Cargo.toml
	[package]
	name = "hello"
	version = "0.1.0"
	authors = ["Vaibhav Gupta <vaibhav.gupta@gmail.com>"]
	edition = "2018"

	[dependencies]
	
	[guptav-virtual-machine 18] > cat src/main.rs
	fn main() {
		println!("Hello, world!");
	}
	
	[guptav-virtual-machine 19] > cargo build
	   Compiling hello v0.1.0 (/home/guptav/repo/rust-circle/hello)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
	
	[guptav-virtual-machine 20] > cargo run
	    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
	     Running `target/debug/hello`
	Hello, world!
	
	[guptav-virtual-machine 21] > cargo check
	    Checking hello v0.1.0 (/home/guptav/repo/rust-circle/hello)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
	
	[guptav-virtual-machine 22] > cargo run
	    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
	     Running `target/debug/hello`
	Hello, world!
	

