#!/bin/bash

cargo build
RUST_LOG=debug,cranelift_codegen=info,wasmtime_cranelift=info,wasmtime=info,wasmtime_wasi=error,tracing=error ./target/debug/wasi-diff -c ${@:1}
