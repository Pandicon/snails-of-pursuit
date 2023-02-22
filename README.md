# Snails of pursuit
This repository contains program for the visualising the paths of the snails from the "Snails of pursuit" question (see below).
![image](https://user-images.githubusercontent.com/70060103/220778008-daa4eb4d-4677-4ad6-8893-9c809e8a8b96.png)

## Screenshots
![image](https://user-images.githubusercontent.com/70060103/220782943-9d3d2ce2-6572-41a0-bc5e-59967935e754.png)
A simulation of 4 snails chasing each other

![image](https://user-images.githubusercontent.com/70060103/220783305-63007689-a3a7-40f4-b3e7-08817f9c4085.png)
A simulation of 13 snails

![image](https://user-images.githubusercontent.com/70060103/220783993-d2f70df0-3da5-4381-8875-cf69d3afea1c.png)
It is also possible to make the snails run away from each other by making their speed negative

## Running
To run the program, either download the [latest release](https://github.com/Pandicon/snails-of-pursuit/releases/latest) (only available for Windows) or compile the source code yourself (see instructions below). Then you can simply double-click the executable or run it from the command line to launch the application.

## Compiling
The easiest way to compile the source code is using Cargo from the [Rust programming language](https://www.rust-lang.org/) toolchain. You can download the toolchain [here](https://www.rust-lang.org/learn/get-started). Once you set it up, you can compile the source code by running `cargo build --release` in the root directory (the one containing the `Cargo.toml` file). This will produce the binary that can be then ran. It is also possible to build and run the binary right after with `cargo run --release`, which both compiles the source code and runs the new binary.
