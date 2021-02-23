# Backstory

Near the end of 2020 I was evaluating my options and wanted to learn OOP, or at least a new systems-level programming language and was evaluating my options. At the time I was still having doubts about C++ and was figuring out the viability of Rust. After discovering that Rust could simply allow me to download a runtime and get started immediately in WSL and VS Code (as opposed to downloading a very large and unintuitive Visual Studio), I was sold on Rust.

# What to expect here?

This repo is an accumulation of notes/exercises/programs accumulated from teaching myself via various online means.

My background is that of a mostly self-taught amateur programmer who's done hobbyist stuff in Python, R, Julia and as such I know enough to bash out a script for the most part, but with no OOP experience. Most of my summary notes there will mostly pick up on what's 'new' to me in Rust.

# Summary Notes

## "The Book"

Link: https://doc.rust-lang.org/book/ 

### Chapter 1 - Basics

Build system and package manager is called `cargo`. Convenient!

### Chapter 2 - Guessing Game

* Adding dependencies to `Cargo.toml` e.g. `rand = "0.5.5"` means add rand version 0.5.5. Recompile to add the dependency to the project
* `std::io` - standard library
* `let mut guess = String::new()` - initialising a mutable variable as a new String 
* Basic error handling with `.expect` - prints result if error, but usually we want to replace this with more precise code.
* `Cargo.lock` freezes dependencies for consistency
* Shadowing - reusing a variable without creating a new variable. The reason we do this is that if we change a mutable variable, it actually assigns another block of memory to the new value, but the old value still remains IN memory! See [this answer](https://stackoverflow.com/questions/53235334/in-rust-whats-the-difference-between-shadowing-and-mutability)

# Other Notes

## Compiling from Linux/WSL to Windows

[Source](https://stackoverflow.com/a/62853319)

### Setup (One-off install via `cargo`, does not require changing config)

Need to install mingw64 (Windows-specific compiler) first: `sudo apt-get install mingw64`

Then we need to add the appropriate toolchain to Rust so that cargo knows where to look
```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```

### Usage

`cargo build --target x86_64-pc-windows-gnu`