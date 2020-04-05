use crate::types::*;
use std::io;

pub trait Canvas {
    fn top_left_corner(&self) -> io::Result<Position>;
    fn bottom_right_corner(&self) -> io::Result<Position>;
    fn size(&self) -> io::Result<Size> {
        let (x1, y1) = self.top_left_corner()?;
        let (x2, y2) = self.bottom_right_corner()?;
        Ok(((x2 - x1), (y2 - y1)))
    }

    fn center(&self) -> io::Result<Position> {
        let (x, y) = self.top_left_corner()?;
        let (w, h) = self.size()?;
        Ok((x + w / 2, y + h / 2))
    }

    fn top_right_corner(&self) -> io::Result<Position> {
        let (_, y) = self.top_left_corner()?;
        let (x, _) = self.bottom_right_corner()?;
        Ok((x, y))
    }

    fn bottom_left_corner(&self) -> io::Result<Position> {
        let (x, _) = self.top_left_corner()?;
        let (_, y) = self.bottom_right_corner()?;
        Ok((x, y))
    }

    fn left_boundary(&self, y: u16) -> io::Result<Position> {
        let (x, _) = self.top_left_corner()?;
        Ok((x, y))
    }

    fn right_boundary(&self, y: u16) -> io::Result<Position> {
        let (x, _) = self.bottom_right_corner()?;
        Ok((x, y))
    }

    fn top_boundary(&self, x: u16) -> io::Result<Position> {
        let (_, y) = self.top_left_corner()?;
        Ok((x, y))
    }

    fn bottom_boundary(&self, x: u16) -> io::Result<Position> {
        let (_, y) = self.bottom_right_corner()?;
        Ok((x, y))
    }

    fn is_boundary(&self, position: &Position) -> io::Result<bool> {
        let (x1, y1) = self.top_left_corner()?;
        let (x2, y2) = self.bottom_right_corner()?;
        let &(px, py) = position;
        Ok(x1 == px || x2 == px || y1 == py || y2 == py)
    }

    fn vcenter(&self, x: u16) -> io::Result<Position> {
        let (_, y) = self.center()?;
        Ok((x, y))
    }

    fn hcenter(&self, y: u16) -> io::Result<Position> {
        let (x, _) = self.center()?;
        Ok((x, y))
    }

    fn covers(&self, location: &Position) -> io::Result<bool> {
        let (x1, y1) = self.top_left_corner()?;
        let (x2, y2) = self.bottom_right_corner()?;
        let (p1, p2) = location;
        Ok(p1 >= &x1 && p1 <= &x2 && p2 >= &y1 && p2 <= &y2)
    }

    fn can_contain<T: Canvas>(&self, canvas: &T) -> io::Result<bool> {
        let c1 = self.covers(&canvas.top_left_corner()?)?;
        let c2 = self.covers(&canvas.bottom_right_corner()?)?;
        Ok(c1 && c2)
    }
}

#[cfg(test)]
mod tests {

    use crate::border::simple_border;
    use crate::canvas::Canvas;
    use crate::types::*;
    use ansi_term::Style;
    use std::io;

    struct MyCanvas {}

    impl Canvas for MyCanvas {
        fn top_left_corner(&self) -> io::Result<Position> {
            Ok((1, 1))
        }
        fn bottom_right_corner(&self) -> io::Result<Position> {
            Ok((10, 10))
        }
    }

    #[test]
    fn it_works() {
        let cv = MyCanvas {};
        let model = "";

        assert_eq!(
            simple_border(&cv, &model, &(1, 1)).unwrap(),
            Some(('┌', Style::default())),
        );
    }
}
