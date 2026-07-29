use super::linear::Linear;
use super::sigmoid::Sigmoid;


pub struct WakeWordClassifier {

    linear: Linear,

    sigmoid: Sigmoid,

}



impl WakeWordClassifier {


    pub fn new() -> Self {

        Self {

            linear:
                Linear::new(
                    32,
                    1,
                ),

            sigmoid:
                Sigmoid::new(),

        }

    }



    pub fn forward(
        &self,
        features: &[f32],
    ) -> f32 {


        let logits =
            self.linear.forward(
                features
            );


        self.sigmoid.forward(
            logits[0]
        )

    }

}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_classifier() {


        let classifier =
            WakeWordClassifier::new();


        let features =
            vec![
                0.0;
                32
            ];


        let score =
            classifier.forward(
                &features
            );


        assert!(
            score >= 0.0 &&
            score <= 1.0
        );

    }

}