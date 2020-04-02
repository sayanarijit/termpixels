use crate::ansi_term::Style;
use std::io::{self, Read, Write};
use termion::event::Event;
use termion::input::Events;
use termion::raw::RawTerminal;

pub type Location = (u16, u16); // width, height

pub type Size = (u16, u16); // x, y

pub trait TermObject {
    fn position(&self) -> Location;
    fn set_position(&mut self, location: &Location);
    fn size(&self) -> Size;
    fn set_size(&mut self, size: &Size);

    fn center(&self) -> Location {
        let position = self.position();
        let size = self.size();

        (position.0 + size.0 / 2, position.1 + size.1 / 2)
    }

    fn is_center(&self, location: &Location) -> bool {
        location == &self.center()
    }

    fn set_center(&mut self, location: &Location) {
        let &(mut x, mut y) = location;
        let (w, h) = self.size();

        x -= w / 2;
        y -= h / 2;

        self.set_position(&(x, y));
    }

    fn top_left_corner(&self) -> Location {
        let (x, y) = self.position();
        (x, y)
    }

    fn is_top_left_corner(&self, location: &Location) -> bool {
        location == &self.top_left_corner()
    }

    fn bottom_left_corner(&self) -> Location {
        let (x, y) = self.position();
        let (_, height) = self.size();

        (x, y + height)
    }

    fn is_bottom_left_corner(&self, location: &Location) -> bool {
        location == &self.bottom_left_corner()
    }

    fn top_right_corner(&self) -> Location {
        let (x, y) = self.position();
        let (width, _) = self.size();

        (x + width, y)
    }

    fn is_top_right_corner(&self, location: &Location) -> bool {
        location == &self.top_right_corner()
    }

    fn bottom_right_corner(&self) -> Location {
        let (x, y) = self.position();
        let (width, height) = self.size();

        (x + width, y + height)
    }

    fn is_bottom_right_corner(&self, location: &Location) -> bool {
        location == &self.bottom_right_corner()
    }

    fn is_corner(&self, location: &Location) -> bool {
        self.is_top_right_corner(location)
            || self.is_top_left_corner(location)
            || self.is_bottom_right_corner(location)
            || self.is_bottom_left_corner(location)
    }

    fn left_boundary(&self) -> u16 {
        self.position().0
    }

    fn is_left_boundary(&self, location: &Location) -> bool {
        location.0 == self.left_boundary()
    }

    fn top_boundary(&self) -> u16 {
        self.position().1
    }

    fn is_top_boundary(&self, location: &Location) -> bool {
        location.1 == self.top_boundary()
    }

    fn bottom_boundary(&self) -> u16 {
        self.position().1 + self.size().1
    }

    fn is_bottom_boundary(&self, location: &Location) -> bool {
        location.1 == self.bottom_boundary()
    }

    fn right_boundary(&self) -> u16 {
        self.position().0 + self.size().0
    }

    fn is_right_boundary(&self, location: &Location) -> bool {
        location.0 == self.right_boundary()
    }

    fn is_boundary(&self, location: &Location) -> bool {
        self.is_right_boundary(location)
            || self.is_left_boundary(location)
            || self.is_top_boundary(location)
            || self.is_bottom_boundary(location)
    }

    fn covers(&self, location: &Location) -> bool {
        let &(x, y) = location;
        x >= self.left_boundary()
            && x <= self.right_boundary()
            && y >= self.top_boundary()
            && y <= self.bottom_boundary()
    }
}

pub trait Shape: TermObject {
    fn ascii_for(&self, _location: &Location) -> char;
}

pub trait Appearance: TermObject {
    fn style_for(&self, _location: &Location) -> Style;
}

pub trait EventHandler {
    fn on_event(&mut self, event: Event) -> io::Result<()>;
}

pub trait Paint: TermObject + Shape + Appearance {
    fn paint(&self, stdout: &mut RawTerminal<io::Stdout>, location: Location) -> io::Result<()>;

    fn paint_all(&self, stdout: &mut RawTerminal<io::Stdout>) -> io::Result<()> {
        for y in self.top_boundary()..(self.bottom_boundary() + 1) {
            for x in self.left_boundary()..(self.right_boundary() + 1) {
                self.paint(stdout, (x, y))?;
            }
        }
        write!(stdout, "{}", termion::cursor::Show)?;
        stdout.flush()
    }
}

pub trait Clear: TermObject + Shape + Appearance {
    fn clear(&self, stdout: &mut RawTerminal<io::Stdout>, location: Location) -> io::Result<()>;

    fn clear_all(&self, stdout: &mut RawTerminal<io::Stdout>) -> io::Result<()> {
        for y in self.top_boundary()..(self.bottom_boundary() + 1) {
            for x in self.left_boundary()..(self.right_boundary() + 1) {
                self.clear(stdout, (x, y))?;
            }
        }
        write!(stdout, "{}", termion::cursor::Show)?;
        stdout.flush()
    }
}

pub trait Render: Paint + Clear + EventHandler {
    fn render<R: Read>(
        &mut self,
        stdout: &mut RawTerminal<io::Stdout>,
        events: &mut Events<R>,
    ) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
