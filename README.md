# WASM - Bevy communication

Trying to upload data via a channel.

> Status: not working

* The sender is owned by a closure called on_click that uploads and `Serializes` the file.
* The receiver is owned by bevy's runtime as a resource. A system listens to it.

Build (using nightly and wasm-bindgen-cli):

```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./pkg --target web ./target/wasm32-unknown-unknown/debug/wasmcomm.wasm
# open the HTML on the browser
xdg-open index.html
```
