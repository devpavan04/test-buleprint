# Number Squarer Blueprint

A simple Tangle Blueprint that squares a number and logs the result.

## Overview

This blueprint demonstrates the basic structure of a Tangle Blueprint with a single job that:
1. Takes an integer as input
2. Squares the number
3. Logs both the input and result
4. Returns the squared value

## Project Structure

- `number-squarer-bin/` - Binary crate containing the runner
- `number-squarer-lib/` - Library crate containing jobs and context
  - `src/context.rs` - Blueprint context definition
  - `src/jobs/square_number.rs` - Job implementation

## Building

```bash
cargo build --release
```

## Running

To run the blueprint locally:

```bash
cargo run --bin number-squarer
```

## Job Details

- **Job ID**: 0
- **Input**: Single integer (i64)
- **Output**: Squared integer (i64)
- **Side Effect**: Logs the operation to console

The job can be triggered by submitting a transaction to the Tangle network with the appropriate service call.
