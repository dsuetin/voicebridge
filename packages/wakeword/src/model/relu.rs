use super::tensor::Tensor;


pub struct ReLU;



impl ReLU {


    pub fn new() -> Self {

        Self

    }



    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Tensor {


        let mut output =
            input.clone();



        for value in output.data.iter_mut() {

            if *value < 0.0 {

                *value = 0.0;

            }
        }


        output

    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::tensor::Tensor;


    #[test]
    fn test_relu() {


        let input =
            Tensor::from_vec(
                vec![
                    -1.0,
                    2.0,
                    -3.0,
                    4.0,
                ],
                2,
                2,
                1,
            );


        let relu =
            ReLU::new();


        let output =
            relu.forward(
                &input
            );


        assert_eq!(
            output.data,
            vec![
                0.0,
                2.0,
                0.0,
                4.0
            ]
        );

    }
}