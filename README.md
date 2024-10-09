# Rust Design Patterns: A Practical Repository

<div align="center">
<img alt="rust image" src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcROkp_bwc1_Apl1nr3RxnO948fSTHE4xcA-5M9B58TDoDqrNKXf_VE4RLJ9kbu_uWWMbcY&usqp=CAU">
</div>


## Description:

This repository is a collection of practical exercises designed to explore the implementation of various design patterns using the Rust programming language. The primary goal is to provide clear and concise examples of how to apply these patterns in real-world projects, leveraging Rust's features and paradigms.

## Implemented Patterns:

* ### Command
We implement a text editor component with two functions: the first is to add characters and second is to enclose all text into spetial characters like braces, brackets, quotes, single quotes or parenthesis. Both functions implement the "Command" trait and have the rollback method.
* ### Factory
We implement a text editor loader that uses two sources: txt and csv, each format has its own factory implementation and returns the object that implements the Document Trait.
* ### Abstract Factory 
We implement two family of geographic entities: the geojson and wkt family. Each family have defined the point, linestring and polygon geometries. This program will have an abstraction to create this objects.



## Project Structure:
src/main: Contains the bootstrap application with the main function. In this function you can start uncomment anything pattern function.

Each pattern its inside from src folder, separated by folders with a pattern name.

## How to use:

### download
use git command:

```bash
git clone https://github.com/miguelonCoder/design_patterns_rust.git
```
### Run
use cargo:
```bash
cargo run
```

