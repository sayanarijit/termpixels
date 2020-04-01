use std::io;
use termpixels::ansi_term::{Color, Style};
use termpixels::termion::event::{Event, Key};
use termpixels::termion::input::TermRead;
use termpixels::termion::raw::IntoRawMode;
use termpixels::{Location, Renderable, Size};

struct GreenBox {
    fill: char,
    size: Size,
    position: Location,
    display: char,
}

impl Renderable for GreenBox {
    fn size(&self) -> &Size {
        &self.size
    }

    fn position(&self) -> &Location {
        &self.position
    }

    fn ascii_for(&self, location: &Location) -> char {
        if &self.center() == location {
            self.display
        } else {
            self.fill
        }
    }

    fn style_for(&self, location: &Location) -> Style {
        if &self.center() == location {
            Style::default().fg(Color::Black).on(Color::White)
        } else {
            Style::default().on(Color::Green)
        }
    }

    fn show_cursor_for(&self, location: &Location) -> bool {
        location == &(2 as u16, 2 as u16) || location == &(2 as u16, 3 as u16)
    }

    fn on_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(Key::Esc) | Event::Key(Key::Ctrl('c')) => {
                Err(io::Error::from(io::ErrorKind::Interrupted))
            }
            Event::Key(Key::Char(c)) => {
                self.display = c;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn main() {
    let mut panel = GreenBox {
        display: '0',
        fill: ' ',
        size: (20, 10),    // width, height
        position: (20, 4), // x, y
    };
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut events = stdin.events();
    if let Err(err) = panel.render(&mut stdout, &mut events) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
