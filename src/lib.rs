// Based of Wasm By Example at https://wasmbyexample.dev/examples/reading-and-writing-audio/reading-and-writing-audio.rust.en-us.html
// Kaitlin Berryman

// wasm_bindgen builds and generates the JS binding
use wasm_bindgen::prelude::*;

// Define the number of samples we can handle at once
const NUM_SAMPLES: usize = 1024;

// Create static mutable byte buffers
// This will pass audio files between JS and Wasm
// IMPORTANT NOTE - using this gloabl 'static mut' means we have UNSAFE code
// but we're proceeding carefully
static mut INPUT_BUFFER: [u8; NUM_SAMPLES] = [0; NUM_SAMPLES];
static mut OUTPUT_BUFFER: [u8; NUM_SAMPLES] = [0; NUM_SAMPLES];

// Function returns a pointer to our output buffer in wasm memory
#[wasm_bindgen]
pub fn get_input_buffer_pointer() -> *const u8{
    let pointer: *const u8;

    // UNSAFE!
    unsafe{
        pointer = INPUT_BUFFER.as_ptr();
    }

    return pointer;
}

// Function that does the amplification
// Take the samples in the input buffers, then amplify them
// then place the result in the output buffer
#[wasm_bindgen]
pub fn amplify_audio(){

    // Loop over samples
    for num in 0..NUM_SAMPLES{
        // STEP ONE - read input buffer

        // Load the sample at the index
        let mut audio_sample: u8;

        // UNSAFE!
        unsafe{
            audio_sample = INPUT_BUFFER[num];
        }

        ////////////////////////////////

        // STEP TWO - amplify each sample one at a time
        // Implemented as bytes. Reference byte samples below
        // 127 is silence. 0 is negative max. 256 is positive max.

        // if audio is positive
        if audio_sample > 127 {
            let audio_sample_diff = audio_sample - 127;
            audio_sample = audio_sample + audio_sample_diff;
        }

        // if audio is negative
        else if audio_sample < 127{

            audio_sample = audio_sample / 2;
        }


        ////////////////////////////////

        // STEP THREE - store audio sample in output buffer
        // UNSAFE!
        unsafe{
            OUTPUT_BUFFER[num] = audio_sample;
        }
    }
}
