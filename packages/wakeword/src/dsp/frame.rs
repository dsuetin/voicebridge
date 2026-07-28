pub struct FrameBuffer {

    buffer: Vec<f32>,

    frame_size: usize,

    hop_size: usize,
}



impl FrameBuffer {


    pub fn new(
        frame_size: usize,
        hop_size: usize,
    ) -> Self {

        Self {
            buffer: Vec::new(),
            frame_size,
            hop_size,
        }

    }



    pub fn push(
        &mut self,
        samples: &[f32],
    ) -> Option<Vec<f32>> {

        self.buffer.extend_from_slice(samples);


        web_sys::console::log_1(
            &format!(
                "frame buffer size={}",
                self.buffer.len()
            )
            .into()
        );


        if self.buffer.len() >= self.frame_size {

            let frame =
                self.buffer[0..self.frame_size]
                    .to_vec();


            self.buffer.drain(0..self.hop_size);


            return Some(frame);
        }


        None
    }

}