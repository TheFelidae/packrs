use std::ops::Mul;

pub trait AtlasArea<T: PartialOrd> {
    const AXIS_COUNT: u8;

    fn axis(&self, axis: u8) -> Option<&T>;

    fn volume(&self) -> T;
}

pub struct AtlasArea2D<T: PartialOrd + Mul = i32> {
    pub width: T,
    pub height: T
}

impl<T: PartialOrd + Mul> AtlasArea<T> for AtlasArea2D<T> {
    const AXIS_COUNT: u8 = 2u8;

    fn axis(&self, axis: u8) -> Option<&T> {
        match axis {
            0 => Some(&self.width),
            1 => Some(&self.height),
            _ => None
        }
    }

    fn volume(&self) -> T {
        self.width * self.height
    }
}

pub struct AtlasArea3D<T: PartialOrd + Mul> {
    pub width: T,
    pub height: T,
    pub depth: T
}

impl<T: PartialOrd + Mul> AtlasArea<T> for AtlasArea3D<T> {
    const AXIS_COUNT: u8  = 3;
    fn axis(&self, axis: u8) -> Option<&T> {
        match axis {
            0 => Some(&self.width),
            1 => Some(&self.height),
            2 => Some(&self.depth),
            _ => None
        }
    }

    fn volume(&self) -> T {
        self.width * self.height * self.depth
    }
}
