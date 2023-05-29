use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{ x: x, y: y, z: z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.x
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

macro_rules! impl_binary_op {
    ($target_type:ident $op:ident $op_fn:ident $op_symbol:tt) => {
        impl<'a, 'b> $op<&'a $target_type> for &'b $target_type {
            type Output = $target_type;
            fn $op_fn(self, other: &'a $target_type) -> $target_type {
                $target_type { x: self.x $op_symbol other.x, y: self.y $op_symbol other.y, z: self.z $op_symbol other.z }
            }
        }

        impl $op<$target_type> for $target_type {
            type Output = $target_type;
            fn $op_fn(self, other: $target_type) -> $target_type {
                &self $op_symbol &other
            }
        }


        impl<'a> $op<&'a $target_type> for $target_type {
            type Output = $target_type;
            fn $op_fn(self, other: &'a $target_type) -> $target_type {
                &self $op_symbol other
            }
        }

        impl<'a> $op<$target_type> for &'a $target_type {
            type Output = $target_type;
            fn $op_fn(self, other: $target_type) -> $target_type {
                self $op_symbol &other
            }
        }

        impl<'a> $op<f32> for &'a $target_type {
            type Output = $target_type;

            fn $op_fn(self, other: f32) -> $target_type {
                $target_type { x: self.x $op_symbol other, y: self.y $op_symbol other, z: self.z $op_symbol other }
            }
        }

        impl $op<f32> for $target_type {
            type Output = $target_type;

            fn $op_fn(self, other: f32) -> $target_type {
                &self $op_symbol other
            }
        }

        impl $op<$target_type> for f32 {
            type Output = $target_type;

            fn $op_fn(self, other: $target_type) -> $target_type {
                &other $op_symbol self
            }
        }

        impl <'a> $op<&'a $target_type> for f32 {
            type Output = $target_type;

            fn $op_fn(self, other: &'a $target_type) -> $target_type {
                other $op_symbol self
            }
        }
    };
}

macro_rules! impl_unary_op {
    ($target_type:ident $op:ident $op_fn:ident $op_symbol:tt) => {
        impl<'a> $op for &'a $target_type {
            type Output = $target_type;

            fn $op_fn(self) -> $target_type {
                $target_type { x: $op_symbol self.x, y: $op_symbol self.y, z: $op_symbol self.z }
            }
        }

        impl $op for $target_type {
            type Output = $target_type;

            fn $op_fn(self) -> $target_type {
                $op_symbol &self
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($target_type:ident $op:ident $op_fn:ident $op_symbol: tt) => {
        impl <'a> $op<&'a $target_type> for $target_type {
            fn $op_fn(&mut self, other: &'a $target_type) {
                *self = $target_type { x: self.x $op_symbol other.x, y: self.y $op_symbol other.y, z: self.z $op_symbol other.z };
            }
        }

        impl $op for $target_type {
            fn $op_fn(&mut self, other: $target_type) {
                *self = *self $op_symbol &other;
            }
        }
    };
}

impl_unary_op!(Vec3 Neg neg -);

impl_binary_op!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_op!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);

impl_binary_op!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_op!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);