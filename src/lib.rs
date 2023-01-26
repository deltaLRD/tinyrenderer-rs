pub mod model;
pub mod vec;
pub mod renderer;
pub mod light;
mod r#const;

#[cfg(test)]
mod tests {
    use super::*;
    use vec::Vec2;
    #[test]
    fn test_vec2_cross_product() {
        let v1 = Vec2::<f32> {
            x:3f32,
            y:4f32,
        };
        let v2 = Vec2::<f32> {
            x:1f32,
            y:1f32,
        };
        assert!(v1.cross_product(&v2) == -1f32);
        assert!(v2.cross_product(&v1) == 1f32);
    }
}