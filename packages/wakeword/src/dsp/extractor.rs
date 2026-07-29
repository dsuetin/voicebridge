use super::frame::FrameBuffer;
use super::mel::MelExtractor;
use super::mel_buffer::MelBuffer;
use crate::model::tensor::Tensor;

pub struct FeatureExtractor {

    frame_buffer: FrameBuffer,

    mel: MelExtractor,

    mel_buffer: MelBuffer,
}

impl FeatureExtractor {

    pub fn new() -> Self {

        Self {

            // 25 ms окно
            // 10 ms шаг
            frame_buffer: FrameBuffer::new(
                400,
                160,
            ),

            mel: MelExtractor::new(),

            // 49 кадров ≈ 490 ms
            mel_buffer: MelBuffer::new(49, 40),
        }
    }


    pub fn process(
        &mut self,
        samples: &[f32],
    ) {

        if let Some(frame) =
            self.frame_buffer.push(samples)
        {

            let mel =
                self.mel.extract(
                    &frame,
                );

            self.mel_buffer.push(
                mel,
            );
        }
    }


    pub fn is_ready(
        &self,
    ) -> bool {

        self.mel_buffer.is_ready()

    }


    pub fn len(
        &self,
    ) -> usize {

        self.mel_buffer.len()

    }


    pub fn window(
        &self,
    ) -> &[f32] {

        self.mel_buffer.window()

    }


    pub fn clear(
        &mut self,
    ) {

        self.mel_buffer.clear();

    }

    pub fn to_tensor(
        &self
    ) -> Tensor {


        let window =
            self.window();



        let mut tensor =
            Tensor::new(
                49,
                40,
                1,
            );


        for y in 0..49 {

            for x in 0..40 {


                let index =
                    y * 40 + x;


                tensor.set(
                    y,
                    x,
                    0,
                    window[index],
                );

            }

        }


        tensor
    }

}