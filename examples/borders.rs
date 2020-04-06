use ansi_term::Style;
use std::io;
use termion::terminal_size;
use termpixels::app;
use termpixels::types::*;

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

pub fn view<C: Canvas, M: Model>(
    canvas: &C,
    _m: &M,
    position: &Position,
) -> io::Result<Option<TermPixel>> {
    if position == &canvas.top_left_corner()? {
        Ok(Some(('┌', Style::default())))
    } else if position == &canvas.top_right_corner()? {
        Ok(Some(('┐', Style::default())))
    } else if position == &canvas.bottom_left_corner()? {
        Ok(Some(('└', Style::default())))
    } else if position == &canvas.bottom_right_corner()? {
        Ok(Some(('┘', Style::default())))
    } else if position == &canvas.right_boundary(position.1)?
        || position == &canvas.left_boundary(position.1)?
    {
        Ok(Some(('│', Style::default())))
    } else if position == &canvas.top_boundary(position.0)?
        || position == &canvas.bottom_boundary(position.0)?
    {
        Ok(Some(('─', Style::default())))
    } else {
        Ok(None)
    }
}

fn update(_: &MyCanvas, _: &mut MyModel, _event: &Event<()>) -> io::Result<Event<()>> {
    Ok(Event::NoOp)
}

fn main() {
    app::run(&init, &view, &update, None).unwrap();
}
