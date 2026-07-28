#[derive(Clone)]
pub struct Tensor {

    pub data: Vec<f32>,

    pub h: usize,

    pub w: usize,

    pub c: usize,
}


impl Tensor {


    pub fn new(
        h: usize,
        w: usize,
        c: usize,
    ) -> Self {

        Self {

            data: vec![0.0; h * w * c],

            h,

            w,

            c,

        }
    }



    pub fn from_vec(
        data: Vec<f32>,
        h: usize,
        w: usize,
        c: usize,
    ) -> Self {

        assert_eq!(
            data.len(),
            h * w * c
        );


        Self {

            data,

            h,

            w,

            c,

        }
    }



    #[inline]
    fn index(
        &self,
        y: usize,
        x: usize,
        ch: usize,
    ) -> usize {

        (y * self.w * self.c)
            +
        (x * self.c)
            +
        ch
    }



    pub fn get(
        &self,
        y: usize,
        x: usize,
        ch: usize,
    ) -> f32 {

        let idx =
            self.index(
                y,
                x,
                ch
            );

        self.data[idx]
    }



    pub fn set(
        &mut self,
        y: usize,
        x: usize,
        ch: usize,
        value: f32,
    ) {

        let idx =
            self.index(
                y,
                x,
                ch
            );

        self.data[idx] = value;
    }



    pub fn len(
        &self,
    ) -> usize {

        self.data.len()

    }
}