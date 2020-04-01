use std::io;
use termpixels::ansi_term::{Color, Style};
use termpixels::termion::event::{Event, Key, MouseEvent};
use termpixels::termion::input::{MouseTerminal, TermRead};
use termpixels::termion::raw::IntoRawMode;
use termpixels::{Location, Renderable, Size};

struct GreenBox<'a> {
    fill: char,
    size: &'a mut Size,
    position: &'a mut Location,
    display: char,
}

impl Renderable for GreenBox<'_> {
    fn size(&self) -> &Size {
        self.size
    }

    fn position(&self) -> &Location {
        &self.position
    }

    fn set_position(&mut self, location: &Location) {
        self.position.0 = location.0;
        self.position.1 = location.1;
    }

    fn ascii_for(&self, location: &Location) -> char {
        if self.is_center(location) {
            self.display
        } else if self.is_corner(location) {
            '+'
        } else if self.is_right_boundary(location) || self.is_left_boundary(location) {
            '│'
        } else if self.is_top_boundary(location) || self.is_bottom_boundary(location) {
            '─'
        } else {
            self.fill
        }
    }

    fn style_for(&self, location: &Location) -> Style {
        if self.is_center(location) {
            Style::default().fg(Color::Black).on(Color::White)
        } else if self.is_boundary(location) {
            Style::default().on(Color::Red)
        } else {
            Style::default().on(Color::Green)
        }
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
            Event::Mouse(me) => match me {
                MouseEvent::Hold(x, y) | MouseEvent::Press(_, x, y) => {
                    self.set_center(&(x, y));
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

fn main() {
    let mut panel = GreenBox {
        display: 'x',
        fill: ' ',
        size: &mut (20, 10),    // width, height
        position: &mut (20, 4), // x, y
    };
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let stdin = io::stdin();
    let mut events = stdin.events();
    if let Err(err) = panel.render(&mut stdout, &mut events) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
