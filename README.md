# Line Counter

CLI tool to count the number of lines in a directory or a file.

## Dev

`cargo watch -x 'run -- test-folder'`

## Motivation

The main motivation is to practice building things in Rust. Some other points:

- Counting lines requires system calls and handling lots of potential errors;
- The logic of counting lines in a directory requires recursive calls (I hope I won't regret this);
- Opportunity to practice organising a Rust project into modules.

## Potential ideas / TODOs

- Include or exclude blank lines from the count
- Verbove or concise output
- Split calculation in multiple threads (with benchmark).
