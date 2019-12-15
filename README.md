# GraphQL WASM DataSource

This repo is an example of how to implement a wasm DataSource for [graphql-go-tools](https://github.com/jensneuse/graphql-go-tools).

## prerequisites

```shell script
brew install rustup
rustup-init
rustup update
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Check your rust installation.

```shell script
rustc --version
```

## building

```shell script
cargo +nightly build --target wasm32-unknown-unknown --release
```
