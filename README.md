
This doesn't do anything interesting yet.

# Building

## Wasm

Buliding web requires the `wasm-pack` cli tool, and requires passing
the `web` feature and removing the default `nat` feature.

```
wasm-pack build --target=web -- --features=web --no-default-features
```

## Native

Building native is the default, so doesn't require any special
arguments at the moment.

```
cargo build
```

