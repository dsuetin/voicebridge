use std::fs::File;
use std::io::{Read, Result};



pub struct WeightsLoader;



impl WeightsLoader {


    pub fn load(
        path: &str,
    ) -> Result<Vec<f32>> {


        let mut file =
            File::open(path)?;


        let mut bytes =
            Vec::new();


        file.read_to_end(
            &mut bytes
        )?;



        assert!(
            bytes.len() % 4 == 0,
            "invalid weights file size"
        );



        let mut weights =
            Vec::with_capacity(
                bytes.len() / 4
            );



        for chunk in bytes.chunks_exact(4) {


            let value =
                f32::from_le_bytes(
                    [
                        chunk[0],
                        chunk[1],
                        chunk[2],
                        chunk[3],
                    ]
                );


            weights.push(
                value
            );

        }



        Ok(weights)

    }



    pub fn save(
        path: &str,
        weights: &[f32],
    ) -> Result<()> {


        use std::io::Write;


        let mut file =
            File::create(path)?;



        for value in weights {


            file.write_all(
                &value.to_le_bytes()
            )?;

        }


        Ok(())

    }

}



#[cfg(test)]
mod tests {


    use super::*;


    #[test]
    fn test_weights_roundtrip() {


        let data =
            vec![
                0.1,
                0.2,
                0.3,
                1.5,
            ];



        let path =
            "/tmp/test_weights.bin";



        WeightsLoader::save(
            path,
            &data,
        )
        .unwrap();



        let loaded =
            WeightsLoader::load(
                path
            )
            .unwrap();



        assert_eq!(
            data,
            loaded
        );

    }

}