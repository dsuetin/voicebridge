use super::tensor::Tensor;


pub struct GlobalAveragePool;


impl GlobalAveragePool {


    pub fn new() -> Self {
        Self
    }



    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Vec<f32> {


        let mut output =
            vec![
                0.0;
                input.c
            ];



        for c in 0..input.c {


            let mut sum =
                0.0;



            for y in 0..input.h {


                for x in 0..input.w {


                    sum += input.get(
                        y,
                        x,
                        c,
                    );

                }

            }



            output[c] =
                sum /
                (input.h * input.w) as f32;

        }



        output
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::tensor::Tensor;


    #[test]
    fn test_global_average_pool() {


        let input =
            Tensor::new(
                19,
                15,
                32,
            );


        let pool =
            GlobalAveragePool::new();


        let output =
            pool.forward(
                &input
            );


        assert_eq!(
            output.len(),
            32
        );

    }

}