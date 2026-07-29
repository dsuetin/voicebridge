use super::tensor::Tensor;


pub struct Conv2D {

    pub kernel_h: usize,

    pub kernel_w: usize,

    pub in_channels: usize,

    pub out_channels: usize,


    // [out_channels][kernel_h][kernel_w][in_channels]
    pub weights: Vec<f32>,


    // [out_channels]
    pub bias: Vec<f32>,
}



impl Conv2D {


    pub fn new(
        kernel_h: usize,
        kernel_w: usize,
        in_channels: usize,
        out_channels: usize,
    ) -> Self {


        Self {

            kernel_h,

            kernel_w,

            in_channels,

            out_channels,


            weights: vec![
                0.0;
                kernel_h
                    * kernel_w
                    * in_channels
                    * out_channels
            ],


            bias: vec![
                0.0;
                out_channels
            ],
        }
    }



    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Tensor {


        let out_h =
            input.h - self.kernel_h + 1;


        let out_w =
            input.w - self.kernel_w + 1;



        let mut output =
            Tensor::new(
                out_h,
                out_w,
                self.out_channels,
            );



        for oc in 0..self.out_channels {

            for y in 0..out_h {

                for x in 0..out_w {


                    let mut sum =
                        self.bias[oc];



                    for ky in 0..self.kernel_h {

                        for kx in 0..self.kernel_w {


                            for ic in 0..self.in_channels {


                                let input_value =
                                    input.get(
                                        y + ky,
                                        x + kx,
                                        ic,
                                    );



                                let weight_index =
                                    ((((oc
                                        * self.kernel_h
                                        + ky)
                                        * self.kernel_w
                                        + kx)
                                        * self.in_channels)
                                        + ic);



                                let weight =
                                    self.weights[
                                        weight_index
                                    ];



                                sum +=
                                    input_value
                                    * weight;
                            }
                        }
                    }



                    output.set(
                        y,
                        x,
                        oc,
                        sum,
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
    fn test_conv() {

        let input =
            Tensor::new(
                100,
                40,
                1,
            );


        let conv =
            Conv2D::new(
                3,
                3,
                1,
                8,
            );


        let output =
            conv.forward(
                &input
            );


        assert_eq!(
            output.h,
            47
        );


        assert_eq!(
            output.w,
            38
        );


        assert_eq!(
            output.c,
            8
        );
    }

    #[test]
    fn test_conv_multichannel() {


        let input =
            Tensor::new(
                23,
                19,
                8,
            );


        let conv =
            Conv2D::new(
                3,
                3,
                8,
                16,
            );


        let output =
            conv.forward(
                &input
            );


        assert_eq!(
            output.h,
            21
        );


        assert_eq!(
            output.w,
            17
        );


        assert_eq!(
            output.c,
            16
        );
    }
}