pub struct FrameBuffer {

    buffer: Vec<f32>,

    frame_size: usize,

}


impl FrameBuffer {


    pub fn new(
        frame_size: usize
    ) -> Self {

        Self {
            buffer: Vec::new(),
            frame_size,
        }
    }



    pub fn push(
        &mut self,
        samples: &[f32]
    ) -> Option<Vec<f32>> {


        self.buffer
            .extend_from_slice(samples);



        if self.buffer.len() >= self.frame_size {

            let frame =
                self.buffer
                    .drain(0..self.frame_size)
                    .collect();


            return Some(frame);
        }


        None
    }

}