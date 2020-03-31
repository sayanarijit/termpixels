pub use ansi_term::{Color, Style};
use std::io;
use std::io::Write;
use termion::raw::RawTerminal;

pub type Location = (u16, u16);

pub type Size = (u16, u16);

pub trait Renderable {
    fn position(&self) -> &Location;
    fn size(&self) -> &Size;
    fn ascii_for(&self, location: &Location) -> char;
    fn style_for(&self, location: &Location) -> Style;

    fn paint(&self, stdout: &mut RawTerminal<io::Stdout>, location: Location) -> io::Result<()> {
        let ch = self.ascii_for(&location);
        let style = self.style_for(&location);

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(location.0, location.1),
            style.paint(ch.to_string())
        )
    }

    fn render(&self, stdout: &mut RawTerminal<io::Stdout>) -> io::Result<()> {
        let position = self.position();
        let size = self.size();

        for y in position.0..(position.0 + size.0 + 1) {
            for x in position.1..(position.1 + size.1 + 1) {
                self.paint(stdout, (x, y))?;
            }
        }
        stdout.flush()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
