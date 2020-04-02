use termpixels::prelude::*;
use termpixels_derive::{
    Appearance, BorderedShape, Clear, EventHandler, Paint, Render, TermObject,
};

// A custom struct
#[derive(TermObject, Appearance, EventHandler, Paint, Clear, Render, BorderedShape)]
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
    if let Err(err) = my_obj.render(&mut stdout, &mut events) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
