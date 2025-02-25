

pub trait Distribution<'a>: 'a {
    fn new(params: Vec<&'a [f64]>) -> Self;
}


/// Multidimensional normal distribution specified by means and a covariance matrix.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct FullNormal<'a> {
    pub mean: &'a [f64],
    pub covr: &'a [f64],
}

impl<'a> Distribution<'a> for FullNormal<'a> {
    fn new(
        params: Vec<&'a [f64]>
    ) -> Self {
        Self {
            mean: params[0],
            covr: params[1],
        }
    }
}