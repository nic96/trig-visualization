# Trig Visualization
A trigonometry function visualizer written in [Bevy](https://bevyengine.org/).

## WASM
<img src="https://i.imgur.com/PO2WSgp.gif" alt="drawing" width="400"/>

Check it out here: [jeromyreimer.com/bevy-showcase/trig-visualization](https://jeromyreimer.com/bevy-showcase/trig-visualization/)

Run the following commands to build and run the wasm version:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --features web
wasm-bindgen --out-dir wasm/target --target web target/wasm32-unknown-unknown/debug/trig-visualization.wasm
cp -r assets wasm/
basic-http-server wasm
```
