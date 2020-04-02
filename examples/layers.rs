use std::io;
use termpixels::ansi_term::Style;
use termpixels::prelude::*;
use termpixels::termion::event::{Event, Key};
use termpixels::termion::terminal_size;
use termpixels_derive::{Appearance, BorderedShape, Clear, Paint, Render, TermObject};

struct MyLayer {
    position: Location,
    size: Size,
    // parent: &MyWindow,
    fill_ascii: char,
    fill_style: Style,
}

#[derive(TermObject, Appearance, Paint, Clear, Render, BorderedShape)]
struct MyWindow {
    position: Location,
    size: Size,
    // layers: Vec<&'a MyLayer<'a>>,
}

// impl MyWindow {
//     fn add_layer(self, fill_ascii: char, fill_style: Style) {
//         let layer = MyLayer<'a> {
//             position: self.position,
//             size: (self.size.0 / 2, self.size.1 / 2),
//             parent: self,
//             fill_ascii: fill_ascii,
//             fill_style: fill_style,
//         };
//         self.layers.push(&layer);
//     }
// }

impl EventHandler for MyWindow {
    fn on_event(&mut self, event: termion::event::Event) -> io::Result<()> {
        match event {
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) | Event::Key(Key::Esc) => {
                Err(io::Error::from(io::ErrorKind::Interrupted))
            }
            _ => Ok(()),
        }
    }
}

fn main() {
    let term_size = terminal_size().unwrap();
    let mut window = MyWindow {
        position: (1, 1),
        size: term_size,
        // layers: Vec::new(),
    };
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut events = stdin.events();
    if let Err(err) = window.render(&mut stdout, &mut events) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
