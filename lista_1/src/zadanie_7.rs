//Zadanie 7

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn move_vec(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn move_to(&mut self, new_x: f64, new_y: f64) {
        self.x = new_x;
        self.y = new_y;
    }

    pub fn rotate_90(&mut self) {
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
    }

    pub fn scale(&mut self, factor: f64) {
        if factor > 0.0 {
            self.width *= factor;
            self.height *= factor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
        assert_eq!(rect.perimeter(), 18.0);
    }

    #[test]
    fn test_move_vec() {
        let mut rect = Rectangle::new(1.0, 1.0, 4.0, 5.0);
        rect.move_vec(2.0, 3.0);
        assert_eq!(rect.x, 3.0);
        assert_eq!(rect.y, 4.0);
    }

    #[test]
    fn test_rotate_90() {
        let mut rect = Rectangle::new(0.0, 0.0, 4.0, 6.0);
        rect.rotate_90();
        assert_eq!(rect.width, 6.0);
        assert_eq!(rect.height, 4.0);
    }

    #[test]
    fn test_scale() {
        let mut rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
        rect.scale(2.0);
        assert_eq!(rect.width, 8.0);
        assert_eq!(rect.height, 10.0);
    }
}