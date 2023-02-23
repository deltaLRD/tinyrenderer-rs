use crate::light;
use crate::model::*;
use crate::r#const::*;
use crate::vec::*;
use image::{ImageBuffer, Rgb};
// use rand::prelude::*;
use crate::light::*;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox<T> {
    pub xmax: T,
    pub xmin: T,
    pub ymax: T,
    pub ymin: T,
}

impl<T> BoundingBox<T>
where
    T: Ord + Copy + From<i32>,
{
    pub fn find_bounding_box(v1: &[T], v2: &[T], v3: &[T]) -> BoundingBox<T> {
        BoundingBox {
            xmax: T::max(v1[0], T::max(v2[0], v3[0])),
            xmin: T::min(v1[0], T::min(v2[0], v3[0])),
            ymax: T::max(v1[1], T::max(v2[1], v3[1])),
            ymin: T::min(v1[1], T::min(v2[1], v3[1])),
        }
    }
    pub fn from(v1: [T; 3], v2: [T; 3], v3: [T; 3]) -> BoundingBox<T> {
        BoundingBox::<T>::find_bounding_box(&v1[0..=2], &v2[0..=2], &v3[0..=2])
    }
    pub fn fit_to_screen<U>(&mut self, width: U, hight: U)
    where
        U: std::ops::Sub<i32, Output = T>,
    {
        self.xmax = self.xmax.min(width - 1);
        self.ymax = self.ymax.min(hight - 1);
        self.xmin = self.xmin.max(0.into());
        self.ymin = self.ymin.max(0.into());
    }
}

pub fn triangle_bounding_box(
    v1: [i32; 3],
    v2: [i32; 3],
    v3: [i32; 3],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: [u8; 3],
    z_bffer: &mut Vec<Vec<f32>>,
) {
    let mut boundingbox = BoundingBox::from(v1, v2, v3);
    let shape = [image.width(), image.height()];
    boundingbox.fit_to_screen(shape[0] as i32, shape[1] as i32);
    for x in boundingbox.xmin..=boundingbox.xmax {
        for y in boundingbox.ymin..=boundingbox.ymax {
            let v1_tmp = [v1[0] as f32, v1[1] as f32];
            let v2_tmp = [v2[0] as f32, v2[1] as f32];
            let v3_tmp = [v3[0] as f32, v3[1] as f32];
            let bc = barycentric_coord(v1_tmp, v2_tmp, v3_tmp, [x as f32, y as f32]);

            if bc.x < 0f32 || bc.y < 0f32 || bc.z < 0f32 {
                continue;
            }
            let mut z_now = 0f32;
            z_now += v1[2] as f32 * bc.x;
            z_now += v2[2] as f32 * bc.y;
            z_now += v3[2] as f32 * bc.z;
            if z_bffer[x as usize][y as usize] <= z_now {
                z_bffer[x as usize][y as usize] = z_now;
                image[(x as u32, y as u32)] = Rgb::from(color);
            }
            
        }
    }
    // line(v1[0], v1[1], v2[0], v2[1], image, BLUE);
    // line(v2[0], v2[1], v3[0], v3[1], image, GREEN);
    // line(v3[0], v3[1], v1[0], v1[1], image, RED);
}

pub fn barycentric_coord<T>(v1: [T; 2], v2: [T; 2], v3: [T; 2], v4: [T; 2]) -> Vec3<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + std::ops::AddAssign
        + From<f32>
        + Into<f32>
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign
        + std::cmp::PartialOrd
        + std::ops::Div<Output = T>,
{
    let w = Vec3 {
        x: v2[0] - v1[0],
        y: v3[0] - v1[0],
        z: v1[0] - v4[0],
    }
    .cross_product(&Vec3 {
        x: v2[1] - v1[1],
        y: v3[1] - v1[1],
        z: v1[1] - v4[1],
    });
    if w.z.partial_cmp(&T::from(1f32)).unwrap() == std::cmp::Ordering::Less
        && w.z.partial_cmp(&T::from(-1f32)).unwrap() == std::cmp::Ordering::Greater
    {
        return Vec3 {
            x: T::from(-1f32),
            y: T::from(1f32),
            z: T::from(1f32),
        };
    }
    Vec3 {
        x: T::from(1f32) - (w.x + w.y) / w.z,
        y: w.y / w.z,
        z: w.x / w.z,
    }
}

pub fn triangle(
    v1: [i32; 2],
    v2: [i32; 2],
    v3: [i32; 2],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: [u8; 3],
) {
    if v1[1] == v2[1] && v2[1] == v3[1] {
        return;
    }
    let mut v1 = v1;
    let mut v2 = v2;
    let mut v3 = v3;
    if v1[1] > v2[1] {
        std::mem::swap(&mut v1, &mut v2);
    }
    if v1[1] > v3[1] {
        std::mem::swap(&mut v1, &mut v3);
    }
    if v2[1] > v3[1] {
        std::mem::swap(&mut v2, &mut v3);
    }
    let dy13 = (v3[1] - v1[1]) as f32;
    let dx13 = (v3[0] - v1[0]) as f32;
    let dy12 = (v2[1] - v1[1]) as f32;
    let dx12 = (v2[0] - v1[0]) as f32;
    let dy23 = (v3[1] - v2[1]) as f32;
    let dx23 = (v3[0] - v2[0]) as f32;
    for y in v1[1]..v2[1] {
        let xs = ((y - v1[1]) as f32) / dy13 * dx13 + (v1[0] as f32);
        let mut xs = xs as u32;
        let xe = ((y - v1[1]) as f32) / dy12 * dx12 + (v1[0] as f32);
        let mut xe = xe as u32;
        if xs > xe {
            std::mem::swap(&mut xs, &mut xe);
        }
        for x in xs..=xe {
            image[(x, y as u32)] = Rgb::from(color);
        }
    }
    for y in v2[1]..v3[1] {
        let xs = ((y - v1[1]) as f32) / dy13 * dx13 + (v1[0] as f32);
        let xe = ((y - v2[1]) as f32) / dy23 * dx23 + (v2[0] as f32);
        let mut xs = xs as u32;
        let mut xe = xe as u32;
        if xs > xe {
            std::mem::swap(&mut xs, &mut xe);
        }
        for x in xs..=xe {
            image[(x, y as u32)] = Rgb::from(color);
        }
    }
    line(v1[0], v1[1], v2[0], v2[1], image, BLUE);
    line(v2[0], v2[1], v3[0], v3[1], image, GREEN);
    line(v3[0], v3[1], v1[0], v1[1], image, RED);
}

pub fn draw_model_line(
    model: &Model,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    lights: &Vec<Light>,
    z_buffer: &mut Vec<Vec<f32>>,
) {
    let width = image.width();
    let hight = image.height();
    // let mut rng = rand::thread_rng();
    let light = lights[0];
    const depth: f32 = 255f32;
    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coord: Vec<Vec3<f32>> = vec![];
        let mut world_coord: Vec<Vec3<f32>> = vec![];
        for j in 0..3 {
            let v0 = model.vert(face[j]);
            // let v1 = model.vert(face[(j + 1) % 3]);
            let x0 = (v0[0] + 1f32) * (width as f32) / 2f32;
            let y0 = (v0[1] + 1f32) * (hight as f32) / 2f32;
            let z0 = (v0[2] + 1f32) * (depth / 2f32);
            screen_coord.push(Vec3 {
                x: x0,
                y: y0,
                z: z0,
            });
            world_coord.push(Vec3 {
                x: v0[0],
                y: v0[1],
                z: v0[2],
            });
        }
        // triangle(vs[0], vs[1], vs[2], image, WHITE);
        let n: Vec3<f32> = world_coord[2]
            .sub(&world_coord[0])
            .cross_product(&(world_coord[1].sub(&world_coord[0])))
            .normalize();

        let intensity = n.dot_product(&light.dir);
        if intensity <= 0f32 {
            continue;
        }
        triangle_bounding_box(
            [
                screen_coord[0].x as i32,
                screen_coord[0].y as i32,
                screen_coord[0].z as i32,
            ],
            [
                screen_coord[1].x as i32,
                screen_coord[1].y as i32,
                screen_coord[1].z as i32,
            ],
            [
                screen_coord[2].x as i32,
                screen_coord[2].y as i32,
                screen_coord[2].z as i32,
            ],
            image,
            [
                (intensity * 255f32).clamp(0f32, 255f32) as u8,
                (intensity * 255f32).clamp(0f32, 255f32) as u8,
                (intensity * 255f32).clamp(0f32, 255f32) as u8,
            ],
            z_buffer,
        );
        // triangle_bounding_box(vs[0], vs[1], vs[2], image, [rng.gen::<u8>(),rng.gen::<u8>(),rng.gen::<u8>()]);
    }
}

pub fn line(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: [u8; 3],
) {
    let width = image.width();
    let hight = image.height();
    let mut x1 = x1 as i32;
    let mut x2 = x2 as i32;
    let mut y1 = y1 as i32;
    let mut y2 = y2 as i32;
    let mut steep = false;
    if x1.abs_diff(x2) < y1.abs_diff(y2) {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
        steep = true;
    }
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;
    let derror = f32::abs(dy / dx);
    let mut error = 0.0f32;
    let mut y = y1;
    for x in x1..=x2 {
        if steep {
            image[((y as u32).min(width - 1), (x as u32).min(hight - 1))] = Rgb::from(color);
        } else {
            image[((x as u32).min(width - 1), (y as u32).min(hight - 1))] = Rgb::from(color);
        }

        error += derror;
        if error > 0.5f32 {
            if y2 > y1 {
                y += 1;
            } else {
                y -= 1;
            }
            error -= 1.0f32;
        }
    }
}
