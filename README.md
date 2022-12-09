# WASM - Bevy communication

Trying to upload data via a channel.

* The sender is owned by a closure called on_click that uploads and `Serializes` the file.
* The receiver is owned by bevy's runtime as a resource. A system listens to it.

Run (using nightly and [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner)):

```bash
cargo run --target wasm32-unknown-unknown
```
