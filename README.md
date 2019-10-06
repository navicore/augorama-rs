Augorama (for rust)
==========

```console
# generate docs
cargo doc --no-deps  

# generate bin
cargo build
./target/debug/augorama

# run in developer mode
RUST_LOG=debug cargo watch -i "examples/**" -x run 
```
