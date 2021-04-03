# IOTA Streams experience
A collection of examples how the community expects IOTA Streams to behave.

## Intent
Several questions arose in the IOTA community due to a discrepancy between expected and actual behavior of the IOTA Streams framework. Also the distinction between what is part of the framework and what is part of applications of the framework seems to be not well understood at the moment. This repository is an attempt to collect and document some of these discrepancies and misconceptions as examples.

## Structure
The repository is structured as a Rust workspace revolving around the examples. The examples should be self contained and have a README associated with them explaining the issue and the expected and actual behavior.
In case of significant code duplication between examples, functionality may be factored out into the utils package.

## Usage
To run one of the examples in the examples folder from the root of the repository, use:
```
cargo run --package <EXAMPLE> [-- <OPTIONAL ARGUMENTS>]
```