# IOTA Streams experience
A collection of examples how the community expects IOTA Streams to behave, with the goal to understand how IOTA Streams works through exploration. Feel free to create pull requests if you have your own examples which need explanations, if you have an explanation for questions in existing examples or if you want to improve the code.

## Intent
Several questions arose in the IOTA community due to a discrepancy between expected and actual behavior of the IOTA Streams framework. Also the distinction between what is part of the framework and what is part of applications of the framework seems to be not well understood sometimes. This repository is an attempt to collect and document some of these discrepancies and misconceptions as examples, so we can work on explaining the intended behavior of IOTA Streams or discuss improvements to the framework.

## Structure
The repository is structured as a Rust workspace revolving around succeeding and failing examples. The examples should be mostly self contained and have a README associated with them explaining the issue and the actual and expected behavior.
In case of significant code duplication between examples, functionality may be factored out into the utils package at the root of the repository.

## Usage
To run one of the examples in the examples folder from the root of this repository, use:
```
cargo run --package <EXAMPLE> [-- <OPTIONAL ARGUMENTS>]
```

To run one of the examples from its own subdirectory, use:
```
cargo run <OPTIONAL ARGUMENTS>
```
