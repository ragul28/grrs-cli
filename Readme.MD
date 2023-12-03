# Rust basic :: grss cli

Simple grep clone to learn rust lang.

Referance: [Rust Book: CLI application in rust](https://rust-cli.github.io/book/tutorial/cli-args.html)

## Usage
```
grrs-cli <pettern> <file path>
```

## Dev Guide

* Running the app cli
    ```sh
    cargo run -- main src/main.rs
    ```
* Running the test case
    ```sh
    cargo test
    ```
* Running exact test case
    ```sh
    cargo test find_a_match 
    ```
* Building rust binary or `-r` build for release
    ```sh
    cargo build -r
    ```