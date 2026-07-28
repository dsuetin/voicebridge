use wasm_bindgen::prelude::*;

mod dsp {
    pub mod mel;
    pub mod frame;
    pub mod mel_buffer;
}

use dsp::mel::MelExtractor;
use dsp::frame::FrameBuffer;
use dsp::mel_buffer::MelBuffer;


const RMS_THRESHOLD: f32 = 0.015;



#[wasm_bindgen]
pub struct WakeWordEngine {

    last_rms: f32,

    mel: MelExtractor,

    frame_buffer: FrameBuffer,

    mel_buffer: MelBuffer,

    detected: bool,
}



#[wasm_bindgen]
impl WakeWordEngine {


    #[wasm_bindgen(constructor)]
    pub fn new() -> WakeWordEngine {

        WakeWordEngine {

            last_rms: 0.0,

            mel:
                MelExtractor::new(),


            frame_buffer:
                FrameBuffer::new(
                    400,
                    160
                ),


            mel_buffer:
                MelBuffer::new(
                    49
                ),


            detected:false,
        }

    }



    pub fn process(
        &mut self,
        samples: &[f32],
    ) -> f32 {


        self.last_rms =
            calculate_rms(samples);



        if let Some(frame) =
            self.frame_buffer.push(samples)
        {


            let features =
                self.mel.extract(
                    &frame
                );


            self.mel_buffer.push(
                features
            );


            web_sys::console::log_1(
                &format!(
                    "mel frames={} ready={} rms={:.4}",
                    self.mel_buffer.len(),
                    self.mel_buffer.is_ready(),
                    self.last_rms
                )
                .into()
            );



            if self.mel_buffer.is_ready()
            {
                //
                // пока заглушка
                //
                self.detected = true;
            }

        }


        self.last_rms
    }


    pub fn wake_detected(
        &self
    ) -> bool {

        self.detected
    }




    pub fn reset(
        &mut self
    ) {

        self.detected = false;

        self.mel_buffer.clear();

    }




    pub fn rms(
        &self
    ) -> f32 {

        self.last_rms
    }




    pub fn speech_detected(
        &self
    ) -> bool {

        self.last_rms > RMS_THRESHOLD
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