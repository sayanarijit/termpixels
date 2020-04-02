use termpixels::ansi_term::{Color, Style};
use termpixels::prelude::*;
use termpixels::termion::event::{Event, Key, MouseEvent};
use termpixels::termion::input::MouseTerminal;
use termpixels_derive::{Clear, Paint, Render, TermObject};

#[derive(TermObject, Paint, Clear, Render)]
struct GreenBox {
    fill: char,
    size: Size,
    position: Location,
    display: char,
}

// A custom struct
impl Shape for GreenBox {
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
}
impl Appearance for GreenBox {
    fn style_for(&self, location: &Location) -> Style {
        if self.is_center(location) {
            Style::default().fg(Color::Black).on(Color::White)
        } else if self.is_boundary(location) {
            Style::default().on(Color::Red)
        } else {
            Style::default().on(Color::Green)
        }
    }
}

impl EventHandler for GreenBox {
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
        size: (20, 10),    // width, height
        position: (20, 4), // x, y
    };
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let stdin = io::stdin();
    let mut events = stdin.events();
    if let Err(err) = panel.render(&mut stdout, &mut events) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
