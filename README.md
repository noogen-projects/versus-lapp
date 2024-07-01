# Versus lapp

A simple and powerful lapp to compare anything.

## Prerequisites

To build this project, you need to have the following tools installed:

- Rust: https://www.rust-lang.org/tools/install
- `cargo-make`: https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation
- `wasm-bindgen-cli`: https://github.com/rustwasm/wasm-bindgen?tab=readme-ov-file#install-wasm-bindgen-cli

For installing these tools, execute the following commands:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

cargo install --force cargo-make wasm-bindgen-cli
```

In addition, you need to install the required toolchains and compilation targets:

```shell
rustup toolchain install stable nightly
rustup target add wasm32-unknown-unknown --toolchain stable
rustup target add wasm32-wasi --toolchain nightly
```

To run the application, you need to have `laplace` installed. If you don't have it, you can install it with the following command:

```shell
cargo install --git https://github.com/noogen-projects/laplace laplace_server
```

Optionally, for the client wasm hot-reloading, you also need to have a `cargo-watch` plugin installed:

```shell
cargo install cargo-watch
```

## Building and running

1. First, clone this project repository and enter the project root directory.

2. Then, build the project with `cargo-make`:

```shell
cargo make all
```

Or for a debug build, use the following command:

```shell
cargo make -p debug all
```

The built lapp will appear in the directory `target/dist/versus/`.

3. Finally, run the project with `laplace_server`:

```shell
laplace_server
```

And open the browser to `http://127.0.0.1:8080/versus`.

Optionally, for hot-reloading wasm-client, you can use the following command in the separate terminal:

```shell
cargo make watch
```
