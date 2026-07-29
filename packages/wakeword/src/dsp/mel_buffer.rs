use crate::model::tensor::Tensor;

pub struct MelBuffer {

    window: Vec<f32>,

    mel_size: usize,

    max_frames: usize,

    frames: usize,
}

impl MelBuffer {


    pub fn new(
        max_frames: usize,
        mel_size: usize,
    ) -> Self {

        Self {

            window: vec![0.0; max_frames * mel_size],

            mel_size,

            max_frames,

            frames: 0,
        }
    }



    pub fn push(
        &mut self,
        frame: Vec<f32>,
    ) {

        debug_assert_eq!(
            frame.len(),
            self.mel_size
        );

        if self.frames < self.max_frames {

            let start =
                self.frames * self.mel_size;

            self.window[
                start..
                start + self.mel_size
            ]
                .copy_from_slice(&frame);

            self.frames += 1;

            return;
        }


        self.window.copy_within(
            self.mel_size..,
            0,
        );


        let start =
            (self.max_frames - 1)
                * self.mel_size;

        self.window[
            start..
            start + self.mel_size
        ]
            .copy_from_slice(&frame);
    }


    pub fn is_ready(
        &self,
    ) -> bool {

        self.frames
            >= self.max_frames

    }

    pub fn window(
        &self,
    ) -> &[f32] {

        &self.window

    }

    pub fn len(
        &self,
    ) -> usize {

        self.frames

    }

    pub fn clear(
        &mut self,
    ) {

        self.frames = 0;

    }

    pub fn to_tensor(
        &self
    ) -> Tensor {


        let mut tensor =
            Tensor::new(
                self.max_frames,
                self.mel_size,
                1,
            );


        for y in 0..self.max_frames {


            for x in 0..self.mel_size {


                let index =
                    y * self.mel_size + x;


                let value =
                    self.window[index];


                tensor.set(
                    y,
                    x,
                    0,
                    value,
                );

            }
        }


        tensor

    }
}