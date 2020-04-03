use termpixels::prelude::*;
use termpixels_derive::{Clear, EventHandler, Object, Paint, Render, Update, View};

// A custom struct
#[derive(Object, View, Update, EventHandler, Paint, Clear, Render)]
struct MyObject {
    position: Location,
    size: Size,
}

fn main() {
    let mut my_obj = MyObject {
        position: (1, 1),
        size: (20, 10),
    };
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut events = stdin.events();
    if let Err(err) = my_obj.render(&mut stdout, &mut events, None) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
    my_obj.clear_all(&mut stdout).unwrap();
}
