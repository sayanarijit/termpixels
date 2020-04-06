use ansi_term::{Color, Style};
use std::io;
use termion::terminal_size;
use termpixels::app;
use termpixels::event::{Event, Input, Key, Mouse};
use termpixels::types::*;

struct MyCanvas {
    size: Size,
    bg_style: Style,
}

impl Canvas for MyCanvas {
    fn top_left_corner(&self) -> io::Result<Position> {
        Ok((1, 1))
    }
    fn bottom_right_corner(&self) -> io::Result<Position> {
        Ok(self.size)
    }
}

struct MyInputBox {
    center: (u16, u16),
    value: char,
    font_style: Style,
    bg_style: Style,
}
impl Canvas for MyInputBox {
    fn top_left_corner(&self) -> io::Result<Position> {
        let (x, y) = self.center;
        Ok((x - 5, y - 1))
    }
    fn bottom_right_corner(&self) -> io::Result<Position> {
        let (x, y) = self.center;
        Ok((x + 5, y + 1))
    }
}

struct MyModel {
    input_box: MyInputBox,
}

fn init() -> io::Result<(MyCanvas, MyModel)> {
    let cv = MyCanvas {
        bg_style: Style::default(),
        size: terminal_size()?,
    };
    let model = MyModel {
        input_box: MyInputBox {
            value: 'x',
            center: cv.center()?,
            font_style: Style::default().fg(Color::Black).on(Color::White),
            bg_style: Style::default().on(Color::Green),
        },
    };
    Ok((cv, model))
}

fn update(_: &MyCanvas, model: &mut MyModel, event: &Event<()>) -> io::Result<Event<()>> {
    match event {
        Event::GracefulStop => Ok(Event::Stop),
        Event::Input(Input::Key(k)) => match k {
            Key::Char(c) => {
                model.input_box.value = *c;
                Ok(Event::NoOp)
            }
            Key::Up => {
                let (x, y) = model.input_box.center()?;
                model.input_box.center = (x, y - 1);
                Ok(Event::NoOp)
            }
            Key::Down => {
                let (x, y) = model.input_box.center()?;
                model.input_box.center = (x, y + 1);
                Ok(Event::NoOp)
            }
            Key::Left => {
                let (x, y) = model.input_box.center()?;
                model.input_box.center = (x - 1, y);
                Ok(Event::NoOp)
            }
            Key::Right => {
                let (x, y) = model.input_box.center()?;
                model.input_box.center = (x + 1, y);
                Ok(Event::NoOp)
            }
            _ => Ok(Event::NoOp),
        },
        Event::Input(Input::Mouse(m)) => match m {
            Mouse::Press(_, x, y) | Mouse::Hold(x, y) => {
                model.input_box.center = (*x, *y);
                Ok(Event::NoOp)
            }
            _ => Ok(Event::NoOp),
        },
        _ => Ok(Event::NoOp),
    }
}

fn view(canvas: &MyCanvas, model: &MyModel, position: &Position) -> io::Result<Option<TermPixel>> {
    match model.input_box.covers(position)? {
        true => match position == &model.input_box.center()? {
            true => Ok(Some((model.input_box.value, model.input_box.font_style))),
            _ => Ok(Some((' ', model.input_box.bg_style))),
        },
        _ => Ok(Some((' ', canvas.bg_style))),
    }
}

fn main() {
    app::run(&init, &view, &update, None).unwrap();
}
