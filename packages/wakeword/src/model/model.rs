use super::tensor::Tensor;
use super::network::WakeWordNet;
use super::global_avg_pool::GlobalAveragePool;
use super::classifier::WakeWordClassifier;



pub struct WakeWordModel {

    network: WakeWordNet,

    pool: GlobalAveragePool,

    classifier: WakeWordClassifier,

}



impl WakeWordModel {


    pub fn new() -> Self {


        Self {

            network:
                WakeWordNet::new(),


            pool:
                GlobalAveragePool::new(),


            classifier:
                WakeWordClassifier::new(),

        }

    }



    pub fn predict(
        &self,
        input: &Tensor,
    ) -> f32 {


        let features =
            self.network
                .forward(
                    input
                );


        let vector =
            self.pool
                .forward(
                    &features
                );


        self.classifier
            .forward(
                &vector
            )

    }

}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_full_model() {


        let model =
            WakeWordModel::new();



        let input =
            Tensor::new(
                100,
                40,
                1,
            );


        let score =
            model.predict(
                &input
            );


        assert!(
            score >= 0.0 &&
            score <= 1.0
        );

    }

}