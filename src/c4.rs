pub enum Light {
    RED,
    GREEN,
    YELLOW,
}
pub trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for Light {
    fn duration(&self) -> u32 {
        match *self {
            Light::RED => 10,
            Light::GREEN => 20,
            Light::YELLOW => 3,
        }
    }
}

pub fn sum(arr: &[u32]) -> Option<u32> {
    let mut sum = 0_u32;
    for i in arr {
        match sum.checked_add(*i) {
            Some(v) => {
                sum = v;
            }
            None => {
                return None;
            }
        }
    }
    return Some(sum);
}
pub struct TRIANG {
    pub base: u32,
    pub height: u32,
}
pub struct RECT {
    pub length: u32,
    pub weight: u32,
}

pub struct SQUARE {
    pub length: u32,
}
pub trait Area {
    fn area(&self) -> u32;
}

impl Area for TRIANG {
    fn area(&self) -> u32 {
        self.base * self.height/2
    }
}

impl Area for RECT {
    fn area(&self) -> u32 {
        self.length * self.weight
    }
}

impl Area for SQUARE {
    fn area(&self) -> u32 {
        self.length * self.length
    }
}

pub fn calc<T: Area>(item: T) -> u32 {
    item.area()
}