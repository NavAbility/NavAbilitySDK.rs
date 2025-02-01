

/// Create a Pose3->Pose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
/// between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
///
/// Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Pose3Pose3<T> {
    pub Z: T
}
