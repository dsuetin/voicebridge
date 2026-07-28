use std::collections::VecDeque;


pub struct MelBuffer {

    frames: VecDeque<Vec<f32>>,

    max_frames: usize,

}



impl MelBuffer {


    pub fn new(
        max_frames: usize,
    ) -> Self {

        Self {

            frames: VecDeque::new(),

            max_frames,

        }
    }



    pub fn push(
        &mut self,
        frame: Vec<f32>,
    ) {


        self.frames
            .push_back(frame);



        while self.frames.len()
            > self.max_frames
        {

            self.frames
                .pop_front();

        }
    }



    pub fn is_ready(
        &self
    ) -> bool {

        self.frames.len()
            >= self.max_frames

    }



    pub fn len(
        &self
    ) -> usize {

        self.frames.len()

    }



    pub fn get_window(
        &self
    ) -> Vec<f32> {


        let mut result =
            Vec::new();



        for frame in &self.frames {

            result.extend_from_slice(
                frame
            );

        }


        result

    }



    pub fn clear(
        &mut self
    ) {

        self.frames.clear();

    }

}