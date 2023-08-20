#!/usr/bin/env just --justfile

say-hello name="zach":
  cargo +nightly -Zscript scripts/hello.rs --name {{name}}

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1


## Packages
### zachsbot-discord
