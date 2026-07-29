use wasm_bindgen::prelude::*;

mod dsp {
    pub mod frame;
    pub mod mel;
    pub mod mel_buffer;
    pub mod extractor;
}
mod model {
    pub mod tensor;
    pub mod conv2d;
    pub mod relu;
    pub mod maxpool;
    pub mod network;
    pub mod flatten;
    pub mod linear;
    pub mod sigmoid;
    pub mod global_avg_pool;
    pub mod classifier;
    pub mod model;
    pub mod weights;
}
use dsp::extractor::FeatureExtractor;
use crate::model::model::WakeWordModel;
const RMS_THRESHOLD: f32 = 0.015;

#[wasm_bindgen]
pub struct WakeWordEngine {

    last_rms: f32,

    extractor: FeatureExtractor,

    model: WakeWordModel,

    detected: bool,
}

#[wasm_bindgen]
impl WakeWordEngine {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WakeWordEngine {

        WakeWordEngine {

            last_rms: 0.0,

            extractor: FeatureExtractor::new(),

            model: WakeWordModel::new(),

            detected: false,
        }
    }


    pub fn process(
        &mut self,
        samples: &[f32],
    ) -> f32 {

        self.last_rms =
            calculate_rms(samples);


        self.extractor.process(
            samples
        );


        if self.extractor.is_ready() {

            web_sys::console::log_1(
                &format!(
                    "mel frames={} window={} rms={:.4}",
                    self.extractor.len(),
                    self.extractor.window().len(),
                    self.last_rms
                )
                .into()
            );

            let tensor =
                self.extractor
                    .to_tensor();


            let score =
                self.model
                    .predict(
                        &tensor
                    );


            web_sys::console::log_1(
                &format!(
                    "wake score={:.4}",
                    score
                )
                .into()
            );



            if score > 0.85 {

                self.detected = true;

            }

        }

        self.last_rms
    }


    pub fn wake_detected(
        &self,
    ) -> bool {

        self.detected
    }


    pub fn reset(
        &mut self,
    ) {

        self.detected = false;

        self.extractor.clear();
    }


    pub fn rms(
        &self,
    ) -> f32 {

        self.last_rms
    }


    pub fn speech_detected(
        &self,
    ) -> bool {

        self.last_rms > RMS_THRESHOLD
    }


    pub fn mel_ready(
        &self,
    ) -> bool {

        self.extractor.is_ready()
    }


    pub fn mel_window(
        &self,
    ) -> Vec<f32> {

        self.extractor
            .window().to_vec()
    }


    pub fn mel_frames(
        &self,
    ) -> usize {

        self.extractor.len()
    }
}


fn calculate_rms(
    samples: &[f32],
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