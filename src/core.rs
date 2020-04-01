use ansi_term::Style;
use std::io::{self, Read, Write};
use termion::event::Event;
use termion::input::Events;
use termion::raw::RawTerminal;

pub type Location = (u16, u16); // width, height

pub type Size = (u16, u16); // x, y

pub trait Renderable {
    fn position(&self) -> &Location;
    fn size(&self) -> &Size;
    fn ascii_for(&self, _location: &Location) -> char;
    fn style_for(&self, location: &Location) -> Style;
    fn show_cursor_for(&self, _location: &Location) -> bool;
    fn on_event(&mut self, event: Event) -> io::Result<()>;

    fn center(&self) -> Location {
        let position = self.position();
        let size = self.size();

        (position.0 + size.0 / 2, position.1 + size.1 / 2)
    }

    fn paint(&self, stdout: &mut RawTerminal<io::Stdout>, location: Location) -> io::Result<()> {
        let ch = self.ascii_for(&location);
        let style = self.style_for(&location);

        write!(stdout, "{}", termion::cursor::Goto(location.0, location.1))?;

        if self.show_cursor_for(&location) {
            write!(stdout, "{}", termion::cursor::Show)?;
        } else {
            write!(stdout, "{}", termion::cursor::Hide)?;
        };

        write!(stdout, "{}", style.paint(ch.to_string()))
    }

    fn paint_all(&self, stdout: &mut RawTerminal<io::Stdout>) -> io::Result<()> {
        let position = self.position();
        let size = self.size();

        for y in position.1..(position.1 + size.1 + 1) {
            for x in position.0..(position.0 + size.0 + 1) {
                self.paint(stdout, (x, y))?;
            }
        }
        write!(stdout, "{}", termion::cursor::Show)?;
        stdout.flush()
    }

    fn clear(&self, stdout: &mut RawTerminal<io::Stdout>, location: Location) -> io::Result<()> {
        write!(stdout, "{} ", termion::cursor::Goto(location.0, location.1))
    }

    fn clear_all(&self, stdout: &mut RawTerminal<io::Stdout>) -> io::Result<()> {
        let position = self.position();
        let size = self.size();

        for y in position.1..(position.1 + size.1 + 1) {
            for x in position.0..(position.0 + size.0 + 1) {
                self.clear(stdout, (x, y))?;
            }
        }
        write!(stdout, "{}", termion::cursor::Show)?;
        stdout.flush()
    }

    fn render<R: Read>(
        &mut self,
        stdout: &mut RawTerminal<io::Stdout>,
        events: &mut Events<R>,
    ) -> io::Result<()> {
        loop {
            self.paint_all(stdout)?;
            if let Some(result) = events.next() {
                if let Ok(event) = result {
                    if let Err(err) = self.on_event(event) {
                        match err.kind() {
                            io::ErrorKind::Interrupted => {
                                break self.clear_all(stdout);
                            }
                            _ => {
                                self.clear_all(stdout)?;
                                break Err(err);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
