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

## Notes

- If I were doing this in TypeScript, I'd probably opt in for recursive version of directory reader by default. But after reading up on using recursion in Rust, it seemed to be more idiomatic to implement iterative version with BFS.
  - It's really nice to have collections like `VecDeque` in the standard library.
