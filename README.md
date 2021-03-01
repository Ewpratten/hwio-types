# Hardware I/O Types
[![Crates.io](https://img.shields.io/crates/v/hwio-types)](https://crates.io/crates/hwio-types) [![Documentation](https://docs.rs/hwio-types/badge.svg)](https://docs.rs/hwio-types) ![Build](https://github.com/Ewpratten/hwio-types/workflows/Build/badge.svg)

`hwio-types` is a small Rust library that provides high level types and traits for basic hardware IO used in robotic control software. The goal of this particular library is to provide some common traits I can use while working on other projects so that I can eventually create a common API / toolset between them without needing any major rewrites, or re-implementing the same traits in each project.

## Installation

### From Crates.io

```sh
cargo install hwio-types
```

### From Source

```sh
git clone https://github.com/ewpratten/hwio-types
cd hwio-types
cargo install --path .
```


