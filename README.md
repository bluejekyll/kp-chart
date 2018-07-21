# Kitchen Patrol Charting Application

An example application, written in Rust, by someone with little frontend experience

## Building

- Install Rust

[https://rustup.rs/](rustup.rs)

- Switch to the nightly compiler

```console
$> rustup default nightly
$> rustup toolchain install wasm32-unknown-unknown
```

- Install `cargo-web`

```console
&> cargo install cargo-web
```

- Start a local appserver

```console
&> cargo web start
```

And connect to the local server: [http://[::1]:8000/](http://[::1]:8000/)

## Deploying

```console
$> cargo web deploy
$> rm -r docs/* && cp target/deploy/* docs/
```