# Experiment Leptos

What's it like making a web app with Leptos?

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Just](https://github.com/casey/just) task runner  
    `brew install just`
3. Run the following to setup wasm dev environment on your local machine

```sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

## Run locally

Make sure everything above is done first ☝️.

To build and run server local: `just` or `just default`.

All recipes are in `justfile`.

All these tasks (currently) use [Trunk](https://trunkrs.dev/) WASM Web Application Bundler.

## Some Notes

```sh
# this will generate a full leptos instance for ssr, ssg, and csr.
cargo leptos new --git https://github.com/leptos-rs/start -n project-name

cargo leptos watch
```

## References

- <https://book.leptos.dev/>
