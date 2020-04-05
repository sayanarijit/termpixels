use std::io;
use termion::terminal_size;
use termpixels::app;
use termpixels::types::*;
use termpixels::views::border::simple_border;

struct MyCanvas {}

struct MyModel {}

impl Canvas for MyCanvas {
    fn top_left_corner(&self) -> io::Result<Position> {
        Ok((1, 1))
    }
    fn bottom_right_corner(&self) -> io::Result<Position> {
        terminal_size()
    }
}

fn init() -> io::Result<(MyCanvas, MyModel)> {
    Ok((MyCanvas {}, MyModel {}))
}

fn update(_: &MyCanvas, _: &mut MyModel, _event: &Event<()>) -> io::Result<Event<()>> {
    Ok(Event::NoOp)
}

fn main() {
    app::run(&init, &simple_border, &update, None).unwrap();
}
