
<p align="center">
    This project doesn't do anything yet.
</p>

# Installing

First install `rustc`+`cargo` (with `rustup`): https://rustup.rs/, 
make sure you can execute `cargo --version`, and then install the
project with cargo.

## From GitHub

```
cargo install --git https://github.com/SimonWoodburyForget/minrusty
```

## From Local

```
cargo install --path /path/to/repository
```

# Running

Once installed it'll be available from `cargo` globally:

```
cargo run minrusty
```

# Building

To build from source, just execute the following from the repository:

## Native

Building native is the default, so doesn't require any special
arguments at the moment.

```
cargo build
```

## Wasm

Buliding web requires the `wasm-pack` cli tool, and requires passing
the `web` feature and removing the default `nat` feature.

```
wasm-pack build core --target=web -- --features=web --no-default-features
```
