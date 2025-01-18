#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<isize> {
    pub fn as_usize(self) -> Option<Point<usize>> {
        let x = self.x.try_into().ok()?;
        let y = self.y.try_into().ok()?;

        Some(Point { x, y })
    }
}

impl From<Point<usize>> for Point<isize> {
    fn from(value: Point<usize>) -> Self {
        Point {
            x: value.x.try_into().expect("usize too big"),
            y: value.y.try_into().expect("usize too big"),
        }
    }
}
