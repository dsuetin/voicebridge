pub struct Sigmoid;


impl Sigmoid {


    pub fn new() -> Self {
        Self
    }



    pub fn forward(
        &self,
        x: f32,
    ) -> f32 {

        1.0 / (1.0 + (-x).exp())

    }

}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_sigmoid() {


        let sigmoid =
            Sigmoid::new();


        let a =
            sigmoid.forward(
                0.0
            );


        assert!(
            (a - 0.5).abs() < 0.001
        );


        let b =
            sigmoid.forward(
                10.0
            );


        assert!(
            b > 0.99
        );


        let c =
            sigmoid.forward(
                -10.0
            );


        assert!(
            c < 0.01
        );

    }
}