use super::tensor::Tensor;


pub struct MaxPool2D {

    pub size: usize,

    pub stride: usize,
}


impl MaxPool2D {


    pub fn new(
        size: usize,
        stride: usize,
    ) -> Self {

        Self {

            size,

            stride,

        }
    }



    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Tensor {


        let out_h =
            (input.h - self.size)
            / self.stride
            + 1;


        let out_w =
            (input.w - self.size)
            / self.stride
            + 1;



        let mut output =
            Tensor::new(
                out_h,
                out_w,
                input.c,
            );



        for c in 0..input.c {


            for y in 0..out_h {


                for x in 0..out_w {


                    let mut max =
                        f32::NEG_INFINITY;



                    for ky in 0..self.size {


                        for kx in 0..self.size {


                            let value =
                                input.get(
                                    y * self.stride + ky,
                                    x * self.stride + kx,
                                    c,
                                );


                            if value > max {

                                max = value;

                            }
                        }
                    }



                    output.set(
                        y,
                        x,
                        c,
                        max,
                    );
                }
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
    fn test_maxpool() {


        let input =
            Tensor::new(
                47,
                38,
                8,
            );


        let pool =
            MaxPool2D::new(
                2,
                2,
            );


        let output =
            pool.forward(
                &input
            );


        assert_eq!(
            output.h,
            23
        );


        assert_eq!(
            output.w,
            19
        );


        assert_eq!(
            output.c,
            8
        );
    }
}