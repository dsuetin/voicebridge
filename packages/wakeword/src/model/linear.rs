pub struct Linear {

    pub input_size: usize,

    pub output_size: usize,

    pub weights: Vec<f32>,

    pub bias: Vec<f32>,
}



impl Linear {


    pub fn new(
        input_size: usize,
        output_size: usize,
    ) -> Self {


        Self {

            input_size,

            output_size,


            weights: vec![
                0.0;
                input_size * output_size
            ],


            bias: vec![
                0.0;
                output_size
            ],
        }

    }



    pub fn forward(
        &self,
        input: &[f32],
    ) -> Vec<f32> {


        let mut output =
            vec![
                0.0;
                self.output_size
            ];



        for o in 0..self.output_size {


            let mut sum =
                self.bias[o];



            for i in 0..self.input_size {


                let index =
                    o * self.input_size + i;


                sum +=
                    input[i]
                    *
                    self.weights[index];

            }


            output[o] = sum;

        }


        output
    }

}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_linear() {


        let layer =
            Linear::new(
                4,
                2,
            );


        let input =
            vec![
                1.0,
                2.0,
                3.0,
                4.0,
            ];


        let output =
            layer.forward(
                &input
            );


        assert_eq!(
            output.len(),
            2
        );

    }
}