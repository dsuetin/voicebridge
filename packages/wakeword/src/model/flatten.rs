use super::tensor::Tensor;


pub struct Flatten;


impl Flatten {


    pub fn new() -> Self {
        Self
    }



    pub fn forward(
        &self,
        input: &Tensor,
    ) -> Vec<f32> {

        input.data.clone()

    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::tensor::Tensor;


    #[test]
    fn test_flatten() {


        let input =
            Tensor::new(
                21,
                17,
                16,
            );


        let flatten =
            Flatten::new();


        let output =
            flatten.forward(
                &input
            );


        assert_eq!(
            output.len(),
            21 * 17 * 16
        );

    }
}