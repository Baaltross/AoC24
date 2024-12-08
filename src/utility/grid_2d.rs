use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Coordinates {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<i64> for Coordinates {
    type Output = Self;

    fn mul(self, multiplier: i64) -> Self::Output {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }
}

#[derive(Debug)]
pub struct Grid<ElementType> {
    buffer: Vec<ElementType>,
    width: usize,
}

impl<ElementType : std::clone::Clone> Grid<ElementType> {
    pub fn new(width: usize, height: usize, default_value: ElementType) -> Grid<ElementType> {
        let buffer_size = width * height;
        let mut buffer = Vec::with_capacity(buffer_size);
        buffer.resize(buffer_size, default_value);
        Grid{buffer, width}
    }

    fn convert_coordinates_to_index(&self, coordinates : Coordinates) -> i64 {
        return coordinates.y * self.width as i64 + coordinates.x;
    }

    pub fn get_value(&self, coordinates : Coordinates) -> Option<&ElementType> {
        let test_index = self.convert_coordinates_to_index(coordinates);
    
        // Note test against width since that would wrap around to the next line
        if test_index < 0 || (test_index >= self.buffer.len() as i64)
        || coordinates.x < 0 || coordinates.x >= self.width as i64 {
            return None;
        }
    
        Some(&self.buffer[test_index as usize])
    }

    pub fn set_value(&mut self, coordinates: Coordinates, new_value: ElementType) -> bool {
        let test_index = self.convert_coordinates_to_index(coordinates);
    
        // Note test against width since that would wrap around to the next line
        if test_index < 0 || (test_index >= self.buffer.len() as i64)
        || coordinates.x < 0 || coordinates.x >= self.width as i64 {
            return false;
        }

        self.buffer[test_index as usize] = new_value;
        true
    }

    pub fn num_rows(&self) -> usize {
        self.buffer.len() / self.width as usize
    }

    pub fn num_columns(&self) -> usize {
        self.width
    }

    pub fn num_cells(&self) -> usize {
        self.buffer.len()
    }

    pub fn iter(self: &Self) -> impl Iterator<Item=&ElementType>{
        self.buffer.iter()
    }
}