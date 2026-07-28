use rustfft::{
    FftPlanner,
    num_complex::Complex,
};


const FFT_SIZE: usize = 512;
const SAMPLE_RATE: usize = 16000;
const MEL_BINS: usize = 40;


pub struct MelExtractor {

    filters: Vec<Vec<f32>>,

}


impl MelExtractor {


    pub fn new() -> Self {

        Self {
            filters:
                create_mel_filters(),
        }

    }



    pub fn extract(
        &self,
        samples: &[f32]
    ) -> Vec<f32> {


        let mut input =
            vec![
                Complex::new(0.0, 0.0);
                FFT_SIZE
            ];


        //
        // Hann window + PCM
        //
        for i in 0..FFT_SIZE {

            let sample =
                samples
                    .get(i)
                    .copied()
                    .unwrap_or(0.0);


            let window =
                0.5
                -
                0.5 *
                (
                    2.0 *
                    std::f32::consts::PI *
                    i as f32
                    /
                    FFT_SIZE as f32
                )
                .cos();


            input[i].re =
                sample * window;
        }



        let mut planner =
            FftPlanner::<f32>::new();


        let fft =
            planner
                .plan_fft_forward(
                    FFT_SIZE
                );


        fft.process(
            &mut input
        );



        //
        // power spectrum
        //
        let power: Vec<f32> =
            input
                .iter()
                .take(FFT_SIZE / 2)
                .map(|x| {

                    x.norm_sqr()
                })
                .collect();



        //
        // mel filterbank
        //
        let mut mel =
            Vec::with_capacity(
                MEL_BINS
            );


        for filter in &self.filters {

            let mut sum = 0.0;


            for (i, weight) in filter.iter().enumerate() {

                sum +=
                    power[i] * weight;

            }


            mel.push(
                (sum + 1e-9).ln()
            );
        }


        mel
    }
}



fn create_mel_filters()
    -> Vec<Vec<f32>>
{

    let bins =
        FFT_SIZE / 2;


    let mut filters =
        Vec::new();


    let low =
        hz_to_mel(80.0);


    let high =
        hz_to_mel(
            7600.0
        );


    let points =
        MEL_BINS + 2;



    let mut mel_points =
        Vec::new();


    for i in 0..points {

        let m =
            low
            +
            (high - low)
            *
            i as f32
            /
            (points - 1) as f32;


        mel_points.push(
            mel_to_hz(m)
        );
    }



    let fft_bins: Vec<usize> =
        mel_points
            .iter()
            .map(|hz| {

                (
                    (FFT_SIZE + 1) as f32
                    *
                    hz
                    /
                    SAMPLE_RATE as f32
                )
                as usize

            })
            .collect();



    for m in 1..points-1 {

        let mut filter =
            vec![
                0.0;
                bins
            ];


        let left =
            fft_bins[m-1];


        let center =
            fft_bins[m];


        let right =
            fft_bins[m+1];



        for i in left..center {

            if center != left {

                filter[i] =
                    (i-left) as f32
                    /
                    (center-left) as f32;
            }
        }



        for i in center..right {

            if right != center {

                filter[i] =
                    (right-i) as f32
                    /
                    (right-center) as f32;
            }
        }


        filters.push(filter);

    }


    filters
}



fn hz_to_mel(
    hz: f32
) -> f32 {

    2595.0 *
    (1.0 + hz / 700.0)
        .log10()

}



fn mel_to_hz(
    mel: f32
) -> f32 {

    700.0 *
    (
        10_f32.powf(
            mel / 2595.0
        )
        -
        1.0
    )

}