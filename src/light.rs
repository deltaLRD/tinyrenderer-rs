use crate::vec::*;

#[derive(Debug,Clone,Copy)]
pub struct Light{
    pub dir:Vec3<f32>,
    pub pos:Vec3<f32>,
}