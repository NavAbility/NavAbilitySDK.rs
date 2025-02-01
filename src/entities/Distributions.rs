

/// Multidimensional normal distribution specified by means and a covariance matrix.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct FullNormal<'a> {
    pub mean: &'a [f64],
    pub covr: &'a [f64],
}