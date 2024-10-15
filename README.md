# dips-ex

Elixir implementation of DIPS: a multi-criteria Cantonese segmentation with Dashes, Intermediates, Pipes, and Spaces.

Note: This package is still in beta, there might be breaking changes in the future. Currently, only macOS (Apple Silicon) is supported.

# Usage

See example usage in `test/dips_ex_test.exs`. Here are the core APIs exposed:

```
assert DipsEx.init_model("native/rust/models/electra-small-6-layers-q4_0.gguf") == {}
assert DipsEx.run_model("阿張先生嗰時好nice㗎", "dips_str") == ["阿-張|先生 嗰-時 好 nice 㗎"]
assert DipsEx.run_model("阿張先生嗰時好nice㗎", "fine") == ["阿", "張", "先生", "嗰", "時", "好", "nice", "㗎"]
assert DipsEx.run_model("阿張先生嗰時好nice㗎", "coarse") == ["阿張先生", "嗰時", "好", "nice", "㗎"]
assert DipsEx.run_model("阿張先生嗰時好nice㗎", "dips") == ["S", "D", "P", "I", "S", "D", "S", "S", "I", "I", "I", "S"]
assert DipsEx.free_model() == {}
```

The current implementation is not thread-safe. You should only call `init_model` once as it creates a global model shared by all subsequent calls to `run_model` and `free_model`.

# Implmentation

[Rustler](https://github.com/rusterlium/rustler) is used to talk with a Rust binding of DIPS originally implemented in C/C++. In the future, we will try to abstract the Rust binding into a separate library, so other languages can also take advantage of the Rust interface.
