
use crate::{
    FullNormal,
    Pose3Pose3,
};



#[allow(non_snake_case)]
impl<'a> Pose3Pose3<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}