#[derive(Debug,Copy,Clone)]
pub struct Vec2<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + std::ops::AddAssign
        + Into<f32>
        + std::ops::Sub<Output = T>,
{
    pub x: T,
    pub y: T,
}

#[derive(Debug,Copy,Clone)]
pub struct Vec3<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + std::ops::AddAssign
        + Into<f32>
        + std::ops::Sub<Output = T>,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec2<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + std::ops::AddAssign
        + Into<f32>
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign
{
    pub fn dot_product(&self, other: &Vec2<T>) -> T {
        self.x.mul(other.x).add(self.y.mul(other.y))
    }
    pub fn add(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn add_in_place(&mut self, other: &Vec2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn sub(&self, other:&Vec2<T>) -> Vec2<T>{
        Vec2{
            x:self.x - other.x,
            y:self.y - other.y
        }
    }
    pub fn sub_in_place(&mut self, other:&Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
    pub fn len2(&self) -> T {
        self.dot_product(self)
    }
    pub fn len(&self) -> f32 {
        f32::sqrt(self.len2().into())
    }
    pub fn cross_product(&self, other: &Vec2<T>) -> T {
        self.x * other.y - self.y * other.x
    }
    pub fn normalize(&self) ->Vec2<f32> {
        let len = self.len();
        Vec2 { x: (self.x.into())/len, y: (self.y.into())/len }
    }
    pub fn as_slice(&self) -> [T;2]{
        [self.x, self.y]
    }
}

impl<T> Vec3<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + std::ops::AddAssign
        + Into<f32>
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign,
{
    pub fn add(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn add_in_place(&mut self, other: &Vec3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn sub(&self, other: &Vec3<T>) -> Vec3<T>{
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
    pub fn sub_in_place(&mut self, other: &Vec3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
    pub fn dot_product(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn len2(&self) -> T {
        self.dot_product(self)
    }
    pub fn len(&self) -> f32 {
        f32::sqrt(self.len2().into())
    }
    pub fn cross_product(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y*other.z-self.z*other.y,
            y: self.z*other.x-self.x*other.z,
            z: self.x*other.y-self.y*other.x,
        }
    }
    pub fn normalize(&self) -> Vec3<f32> {
        let len = self.len();
        Vec3{
            x:self.x.into()/len,
            y:self.y.into()/len,
            z:self.z.into()/len
        }
    }
    pub fn as_slice(&self) -> [T;3] {
        [self.x, self.y, self.z]
    }
}
