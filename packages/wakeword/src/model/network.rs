use super::tensor::Tensor;
use super::conv2d::Conv2D;
use super::relu::ReLU;
use super::maxpool::MaxPool2D;



pub struct WakeWordNet {


    conv1: Conv2D,

    relu1: ReLU,

    pool1: MaxPool2D,


    conv2: Conv2D,

    relu2: ReLU,


    conv3: Conv2D,

    relu3: ReLU,

}



impl WakeWordNet {


    pub fn new() -> Self {


        Self {


            conv1:
                Conv2D::new(
                    3,
                    3,
                    1,
                    8,
                ),



            relu1:
                ReLU::new(),



            pool1:
                MaxPool2D::new(
                    2,
                    2,
                ),



            conv2:
                Conv2D::new(
                    3,
                    3,
                    8,
                    16,
                ),



            relu2:
                ReLU::new(),



            conv3:
                Conv2D::new(
                    3,
                    3,
                    16,
                    32,
                ),



            relu3:
                ReLU::new(),

        }

    }




    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Tensor {



        let x =
            self.conv1
                .forward(
                    input
                );



        let x =
            self.relu1
                .forward(
                    &x
                );



        let x =
            self.pool1
                .forward(
                    &x
                );



        let x =
            self.conv2
                .forward(
                    &x
                );



        let x =
            self.relu2
                .forward(
                    &x
                );



        let x =
            self.conv3
                .forward(
                    &x
                );



        let x =
            self.relu3
                .forward(
                    &x
                );



        x

    }

}




#[cfg(test)]
mod tests {


    use super::*;



    #[test]
    fn test_network_forward() {



        let input =
            Tensor::new(
                49,
                40,
                1,
            );



        let net =
            WakeWordNet::new();



        let output =
            net.forward(
                &input
            );



        assert_eq!(
            output.h,
            19
        );



        assert_eq!(
            output.w,
            15
        );



        assert_eq!(
            output.c,
            32
        );

    }

}