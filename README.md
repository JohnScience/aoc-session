# Get the session cookie value for Advent of Code easily

[![Crates.io](https://img.shields.io/crates/v/aoc-session)](https://crates.io/crates/aoc-session)
[![Downloads](https://img.shields.io/crates/d/aoc-session.svg)](https://crates.io/crates/aoc-session)
[![Documentation](https://docs.rs/aoc-session/badge.svg)](https://docs.rs/aoc-session)
[![License](https://img.shields.io/crates/l/aoc-session)](https://crates.io/crates/aoc-session)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/aoc-session/status.svg)](https://deps.rs/repo/github/JohnScience/aoc-session)

## Usage (library)

```rust
use aoc_session::aoc_session;

fn main() {
    let session_id: String = match aoc_session() {
        // Ok(session) => <aoc_session::AocSession as ToString>::to_string(&session),
        Ok(session) => session.to_string(),
        Err(e) => panic!("Error: {e}"),
    };

    assert!(session_id.len() > 0);
    println!("My session ID: {}", session_id);
}
```

## Installation (executable)

```console
cargo install aoc-session
```

## Usage (executable)

```console
aoc-session
```

## Learn more

This crate was born while preparing [the presentation](https://docs.google.com/presentation/d/1v5IM6GbMgvC2EVPQnJ1-adwQONMmLOWmDeGGE6Ehx-U/edit?usp=sharing) for Calgary Rust community. It better explains what this crate provides and how it works.
