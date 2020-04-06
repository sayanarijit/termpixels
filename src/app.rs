use crate::canvas::Canvas;
use crate::event::{Event, Input, Key};
use crate::exit_code::ExitCode;
use crate::types::*;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use termion::async_stdin;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn render<C: Canvas, M: Model, V: View<C, M>>(
    canvas: &C,
    model: &M,
    view: &V,
) -> io::Result<Vec<(Position, TermPixel)>> {
    let mut vec: Vec<(Position, TermPixel)> = Vec::new();

    let (x1, y1) = canvas.top_left_corner()?;
    let (x2, y2) = canvas.bottom_right_corner()?;

    for y in y1..(y2 + 1) {
        for x in x1..(x2 + 1) {
            if let Ok(Some(tp)) = view(canvas, model, &(x, y)) {
                vec.push(((x, y), tp));
            }
        }
    }
    Ok(vec)
}

pub fn run<C: Canvas, M: Model, E, I: Init<C, M>, V: View<C, M>, U: Update<C, M, E>>(
    init: &I,
    view: &V,
    update: &U,
    refresh_interval: Option<Duration>,
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = MouseTerminal::from(stdout.lock().into_raw_mode()?);
    let mut inputs = async_stdin().events();
    let mut interrupted = false;
    let mut screen: HashMap<Position, TermPixel> = HashMap::new();

    let (canvas, mut model) = init()?;
    let mut updates = render(&canvas, &model, view)?;
    let mut event = Event::NoOp;

    write!(stdout, "{}", termion::cursor::Hide)?;
    loop {
        if !interrupted {
            if updates.len() != 0 as usize {
                for (position, (ascii, style)) in updates.iter() {
                    let &(x, y) = position;
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Goto(x, y),
                        style.paint(ascii.to_string())
                    )?;
                }

                stdout.flush()?;
                updates.clear();
            }

            for (position, (ascii, style)) in render(&canvas, &model, view)? {
                if let Some((curr_ascii, curr_style)) = screen.get(&position) {
                    if curr_ascii == &ascii && curr_style == &style {
                        continue;
                    };
                };

                updates.push((position, (ascii, style)));
                screen.insert(position, (ascii, style));
            }
        }

        match event {
            Event::Stop => {
                writeln!(stdout, "{}", termion::cursor::Show)?;
                drop(stdout);
                drop(inputs);
                drop(model);
                std::process::exit(ExitCode::OK as i32);
            }
            Event::NoOp => {
                if let Some(Ok(input)) = inputs.next() {
                    match input {
                        Input::Key(k) => match k {
                            Key::Ctrl('c') => match interrupted {
                                false => {
                                    interrupted = true;
                                    update(&canvas, &mut model, &Event::GracefulStop)?;
                                    event = Event::Stop;
                                }
                                true => {
                                    std::process::exit(ExitCode::ForcefulStop as i32);
                                }
                            },
                            _ => {
                                event = Event::Input(input);
                            }
                        },
                        _ => {
                            event = Event::Input(input);
                        }
                    }
                };
            }
            _ => {}
        }

        if interrupted {
            continue;
        }

        event = update(&canvas, &mut model, &event)?;

        if let Some(interval) = refresh_interval {
            std::thread::sleep(interval);
        }
    }
}
