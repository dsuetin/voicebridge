use rustfft::{
    FftPlanner,
    num_complex::Complex,
};


pub struct MelExtractor {
    fft_size: usize,
    sample_rate: usize,
}


impl MelExtractor {

    pub fn new() -> Self {

        Self {
            fft_size: 512,
            sample_rate: 16000,
        }

    }


    pub fn extract(
        &self,
        samples: &[f32]
    ) -> Vec<f32> {

        let mut input =
            vec![
                Complex::new(0.0, 0.0);
                self.fft_size
            ];


        for (i, sample) in samples
            .iter()
            .take(self.fft_size)
            .enumerate()
        {
            input[i].re = *sample;
        }


        let mut planner =
            FftPlanner::<f32>::new();


        let fft =
            planner.plan_fft_forward(
                self.fft_size
            );


        fft.process(
            &mut input
        );


        let mut spectrum =
            Vec::new();


        for bin in input.iter()
            .take(self.fft_size / 2)
        {

            let power =
                bin.norm_sqr();


            spectrum.push(
                (power + 1e-9).ln()
            );
        }


        spectrum
    }
}