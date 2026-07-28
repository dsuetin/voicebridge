use wasm_bindgen::prelude::*;
mod dsp { pub mod mel; pub mod frame; }
use dsp::mel::MelExtractor;
use dsp::frame::FrameBuffer;

const SAMPLE_RATE: usize = 16000;
const BUFFER_SECONDS: usize = 5;

const RMS_THRESHOLD: f32 = 0.015;



#[wasm_bindgen]
pub struct WakeWordEngine {

    buffer: Vec<f32>,

    max_samples: usize,

    last_rms: f32,

    mel: MelExtractor,

    frame_buffer: FrameBuffer,
}


#[wasm_bindgen]
impl WakeWordEngine {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WakeWordEngine {

        WakeWordEngine {
            buffer: Vec::with_capacity(
                SAMPLE_RATE * BUFFER_SECONDS
            ),
            max_samples: SAMPLE_RATE * BUFFER_SECONDS,
            last_rms: 0.0,
            mel: MelExtractor::new(),
            frame_buffer: FrameBuffer::new(512),
            
        }

    }


    pub fn process(
        &mut self,
        samples: &[f32],
    ) -> f32 {


        self.buffer.extend_from_slice(samples);


        if self.buffer.len() > self.max_samples {

            let excess =
                self.buffer.len() - self.max_samples;

            self.buffer.drain(0..excess);
        }


        self.last_rms =
            calculate_rms(samples);

        // web_sys::console::log_1(
        //     &format!(
        //         "rms={:.4}",
        //         self.last_rms
        //     )
        //     .into()
        // );
        if self.speech_detected() {

            if let Some(frame) =
                self.frame_buffer.push(samples)
            {

                let features =
                    self.mel.extract(
                        &frame
                    );


                web_sys::console::log_1(
                    &format!(
                        "mel size={} frame={}",
                        features.len(),
                        frame.len()
                    )
                    .into()
                );
            }
        }

        0.0
    }


    pub fn rms(&self) -> f32 {
        self.last_rms
    }


    pub fn speech_detected(&self) -> bool {
        self.last_rms > RMS_THRESHOLD
    }


    pub fn buffer_seconds(&self) -> f32 {

        self.buffer.len() as f32
            /
        SAMPLE_RATE as f32
    }


    pub fn samples_count(&self) -> usize {

        self.buffer.len()

    }


    pub fn clear(&mut self) {

        self.buffer.clear();

    }
}



fn calculate_rms(
    samples: &[f32]
) -> f32 {

    if samples.is_empty() {
        return 0.0;
    }


    let sum: f32 =
        samples
            .iter()
            .map(|x| x * x)
            .sum();


    (sum / samples.len() as f32).sqrt()
}