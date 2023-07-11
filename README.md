# `jsfind`

Use `jsfind` to search the current directory recursively for lines that match a JS expression.

Built by embedding the `boa` JS runtime, walking directories using `ignore` to respect `.gitignore`, and parallel iteration with `rayon`.

Inspired by https://austinpoor.com/blog/js-in-rs

# Installation and usage

```
git clone https://github.com/tigregalis/jsfind
cd jsfind
cargo install --path .
jsfind "line.startsWith('fn') && line.endsWith('{')"
# main.rs
# 6:fn main() -> Result<(), Box<dyn std::error::Error>> {
```

# Status

A throwaway project to try out `boa`. As a tool for searching files, it works for my needs.

If I ever extend this, I'd want to:

- [ ] use `clap`
- [ ] have better error handling
- [ ] support mapping results
- [ ] mirror the `ripgrep` interface
- [ ] optimise performance, e.g. don't construct a `boa_engine::Context` with every file (just one per thread)
