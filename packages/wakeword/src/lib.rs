use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WakeWordEngine {
    buffer: Vec<f32>,
    max_samples: usize,
}

#[wasm_bindgen]
impl WakeWordEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            processed_samples: 0,
        }
    }

    pub fn process(&mut self, samples: &[f32]) -> f32 {
        self.processed_samples += samples.len();

        // Пока просто возвращаем "score"
        0.0
    }

    pub fn processed_seconds(&self) -> f32 {
        self.processed_samples as f32 / 16000.0
    }

    pub fn reset(&mut self) {
        self.processed_samples = 0;
    }
}