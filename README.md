# Audio Player
## Web Assembly & Rust

An exercise based on https://wasmbyexample.dev/examples/reading-and-writing-audio/reading-and-writing-audio.rust.en-us.html from Wasm By Example (Specifically Aaron Turner/torch2424)

Note — This demo is just for learning purposes, not any kind of production

### Overview
WebAssembly is great for computationally intensive tasks. Meaning, generating/rendering audio samples can be significantly sped up by implementing them in WebAssembly.

In this exercise, we're amplifying audio samples from an AudioBuffer using the Web Audio API. Remember — Wasm By Example chose this example for demo purposes, not for more realistic use cases

### Implementation
Start by watching [this video](https://www.youtube.com/watch?v=1RIA9U5oXro&ab_channel=Computerphile) on how digital audio works by Computerphile.. unfortunate name lol

Summary — digital audio can be stored by a one dimensional array containing positive (1.0) and negative (-1.0) signals. The index of the array represents time, and the value represents the volume.

### Let's Get started
`cd Desktop`, `mkdir wasm-audio`, `cd wasm-audio`, `cargo new audioplayer`

Open up `src/lib.rs`

Alright let's go line by line

wasm_bindgen builds and generates the JS binding file
import the wasm_bindgen with this line
`use wasm_bindgen::prelude::*;`

Define the number of samples we can handle at once
`const NUMB_SAMPLES: usize = 1024;`

THIS PART IS UNSAFE CODE - BE CAREFUL
It's unsafe because of the `static mut` in global scope
This will help us pass the audio files between JS and Wasm
```
static mut INPUT_BUFFER: [u8; NUM_SAMPLES] = [0; NUM_SAMPLES];
static mut OUTPUT_BUFFER: [u8; NUM_SAMPLES] = [0; NUM_SAMPLES];
```

-------------

`cargo install wasm-pack`
Then, let's edit our new  `Cargo.toml`  to implement  [wasm-pack](https://github.com/rustwasm/wasm-pack#-quickstart-guide)  as mentioned in their quickstart:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

Rename `src/main.rs` to `src/lib.rs`

`wasm-pack build --target web`
