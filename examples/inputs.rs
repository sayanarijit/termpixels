use termpixels::ansi_term::{Color, Style};
use termpixels::prelude::*;
use termpixels::termion::event::{Event, Key, MouseEvent};
use termpixels::termion::input::MouseTerminal;
use termpixels_derive::{Object, Paint, Render, Update};

#[derive(Object, Update, Paint, Render)]
struct GreenBox {
    fill: char,
    fill_style: Style,
    size: Size,
    position: Location,
    display: char,
    display_style: Style,
    border_style: Style,
}

impl View for GreenBox {
    fn ascii_for(&self, location: &Location) -> Option<char> {
        if self.is_center(location) {
            Some(self.display)
        } else if self.is_corner(location) {
            Some('+')
        } else if self.is_right_boundary(location) || self.is_left_boundary(location) {
            Some('│')
        } else if self.is_top_boundary(location) || self.is_bottom_boundary(location) {
            Some('─')
        } else {
            Some(self.fill)
        }
    }

    fn style_for(&self, location: &Location) -> Style {
        if self.is_center(location) {
            self.display_style
        } else if self.is_boundary(location) {
            self.border_style
        } else {
            self.fill_style
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
        display_style: Style::default().fg(Color::Black).on(Color::White),
        fill: ' ',
        fill_style: Style::default().on(Color::Green),
        border_style: Style::default().on(Color::Red),
        size: (20, 10),    // width, height
        position: (20, 4), // x, y
    };

    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let stdin = io::stdin();
    let mut events = stdin.events();

    if let Err(err) = panel.render(&mut stdout, &mut events, None) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
