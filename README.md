# dips-ex

Elixir implementation of DIPS: a multi-criteria Cantonese segmentation with Dashes, Intermediates, Pipes, and Spaces.

Note: This package is still in beta, there might be breaking changes in the future. Currently, only macOS (Apple Silicon) is supported.

# Implmentation

[Rustler](https://github.com/rusterlium/rustler) is used to talk with a Rust binding of DIPS originally implemented in C/C++. In the future, we will try to abstract the Rust binding into a separate library, so other languages can also take advantage of the Rust interface.
