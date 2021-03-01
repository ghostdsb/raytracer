use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign,SubAssign, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    e: [f32;3],
}

impl Vec3{
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self{
        Vec3{
            e: [e0,e1,e2]
        }
    }

    pub fn x(&self) -> f32 {self.e[0]}

    pub fn y(&self) -> f32 {self.e[1]}

    pub fn z(&self) -> f32 {self.e[2]}

    pub fn length_squared(&self) -> f32 {
        self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x()*other.x() + self.y()*other.y() + self.z()*other.z()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * self.e[1],
                self.e[2] * other.e[0] - self.e[0] * self.e[2],
                self.e[0] * other.e[1] - self.e[1] * self.e[0],
            ]
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
    
}

impl Add for Vec3{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3{
            e:[self.x()+other.x(), self.y()+other.y(), self.z()+other.z()]
        }
    }
}

impl Sub for Vec3{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3{
            e:[self.x()-other.x(), self.y()-other.y(), self.z()-other.z()]
        }
    }
}

impl Neg for Vec3{
    type Output = Self;

    fn neg(self) -> Self {
        Vec3{
            e: [-self.x(), -self.y(), -self.z()]
        }
    }
}

impl Index<usize> for Vec3{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output{
        &self.e[index]
    }

}

impl IndexMut<usize> for Vec3{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.e[index]
    }

}

impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Self){
        *self = Vec3{
            e: [self.x()+other.x(), self.y()+other.y(), self.z()+other.z()]
        }
    }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self, other: Self){
        *self = Vec3{
            e: [self.x()-other.x(), self.y()-other.y(), self.z()-other.z()]
        }
    }
}

impl MulAssign<f32> for Vec3{
    fn mul_assign(&mut self, scalar: f32){
        *self = Vec3{
            e: [self.x()*scalar, self.y()*scalar, self.z()*scalar]
        }
    }
}

impl Mul<f32> for Vec3{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vec3{
            e: [self.x()*scalar, self.y()*scalar,self.z()*scalar]
        }
    }
}

impl Mul<Vec3> for Vec3{
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Vec3{
            e: [self.x()*other.x(), self.y()*other.y(),self.z()*other.z()]
        }
    }
}


impl DivAssign<f32> for Vec3{
    fn div_assign(&mut self, scalar: f32){
        *self = Vec3{
            e: [self.x()/scalar, self.y()/scalar, self.z()/scalar]
        }
    }
}

impl Div<f32> for Vec3{
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        self*(1.0/scalar)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_of_vectors() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        let mut vec3 = Vec3::new(3.0, 4.0, 5.0);
        vec3 += vec1;
        assert_eq!(vec1+vec2, Vec3::new(3.0, 5.0, 7.0));
    }
    
    #[test]
    fn subtraction_of_vectors() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        let mut vec4 = Vec3::new(3.0, 4.0, 5.0);
        vec4 -= vec1;
        assert_eq!(vec1-vec2, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(vec4, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn negative_of_vector() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-vec1, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn indexing_of_vector() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec1[0], 1.0);
        assert_eq!(vec1[1], 2.0);
        assert_eq!(vec1[2], 3.0);
    }

    #[test]
    fn scalar_multiplication_of_vector() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = vec1 * 2.0;
        vec1 *= 4.0;
        assert_eq!(vec1[0], 4.0);
        assert_eq!(vec1[1], 8.0);
        assert_eq!(vec1[2], 12.0);
        assert_eq!(vec2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scalar_division_of_vector() {
        let mut vec1 = Vec3::new(16.0, 20.0, 24.0);
        let vec2 = vec1 / 2.0;
        vec1 /= 4.0;
        assert_eq!(vec1[0], 4.0);
        assert_eq!(vec1[1], 5.0);
        assert_eq!(vec1[2], 6.0);
        assert_eq!(vec2, Vec3::new(8.0, 10.0, 12.0));
    }

    #[test]
    fn vector_length() {
        let vec1 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(vec1.length(), ((9+16+25) as f32).sqrt());
    }
}
