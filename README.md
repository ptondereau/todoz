# TODOz

It's currently a work in progress.

⚠️ Disclaimer -- This is a personal project and not intended to be used in production. It is for educational purposes for the moment.

This is a simple Smart Rollup TODO Kernel for Tezos blockchain. It is a simple example of how to write a Smart Rollup Kernel with.

## How to use

Before using this project, please refer to the setup instructions in the [Tezos Gallery repository](https://gitlab.com/tezos/kernel-gallery#setup).

```bash
# Build the kernel
cargo build --release --target wasm32-unknown-unknown

# Simulate it
octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/todo_kernel.wasm --inputs inputs.json
```

