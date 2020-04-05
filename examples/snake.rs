use ansi_term::{Color, Style};
use std::collections::VecDeque;
use std::io;
use std::time::Duration;
use std::time::SystemTime;
use termion::terminal_size;
use termpixels::app;
use termpixels::event::{Event, Input, Key};
use termpixels::types::*;
use termpixels::views::border::simple_border;

struct MyCanvas {
    size: Size,
    fill_ascii: char,
    fill_style: Style,
}

impl Canvas for MyCanvas {
    fn top_left_corner(&self) -> io::Result<Position> {
        Ok((1, 1))
    }
    fn bottom_right_corner(&self) -> io::Result<Position> {
        Ok(self.size)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Msg {
    ChangeDirection(Direction),
    FoodEaten,
    GameOver,
}

struct Snake {
    head_ascii: char,
    head_style: Style,
    body_ascii: char,
    body_style: Style,
    body: VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    fn new(canvas: &MyCanvas) -> io::Result<Snake> {
        let (x, y) = canvas.center()?;
        let mut snake = Snake {
            head_ascii: '⚇',
            head_style: Style::default().fg(Color::Cyan),
            body_ascii: '◌',
            body_style: Style::default().fg(Color::Cyan),
            body: VecDeque::new(),
            direction: Direction::Right,
        };
        snake.body.push_back(((x + 1), y));
        snake.body.push_back((x, y));
        snake.body.push_back(((x - 1), y));
        Ok(snake)
    }
}

struct Food {
    position: Position,
    ascii: char,
    style: Style,
}

impl Food {
    fn new(canvas: &MyCanvas) -> io::Result<Food> {
        let mut bad_rand1 = u16::max_value();
        let mut bad_rand2 = u16::max_value();

        let (w, h) = canvas.size()?;

        while !canvas.covers(&(bad_rand1, bad_rand2))? {
            bad_rand1 = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u16
                % (w - 2)
                + 1;
            bad_rand2 = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u16
                % (h - 2)
                + 1;
        }

        Ok(Food {
            position: (bad_rand1, bad_rand2),
            style: Style::default().fg(Color::Green),
            ascii: '❄',
        })
    }
}

struct MyModel {
    fast: bool,
    snake: Snake,
    food: Food,
}

fn init() -> io::Result<(MyCanvas, MyModel)> {
    let cv = MyCanvas {
        size: terminal_size().unwrap(),
        fill_ascii: ' ',
        fill_style: Style::default().on(Color::Black),
    };
    let model = MyModel {
        fast: false,
        snake: Snake::new(&cv)?,
        food: Food::new(&cv)?,
    };
    Ok((cv, model))
}

fn update(canvas: &MyCanvas, model: &mut MyModel, event: &Event<Msg>) -> io::Result<Event<Msg>> {
    match event {
        Event::GracefulStop => Ok(Event::Stop),
        Event::Input(Input::Key(k)) => match k {
            Key::Up | Key::Char('k') => Ok(Event::Msg(Msg::ChangeDirection(Direction::Up))),
            Key::Down | Key::Char('j') => Ok(Event::Msg(Msg::ChangeDirection(Direction::Down))),
            Key::Left | Key::Char('h') => Ok(Event::Msg(Msg::ChangeDirection(Direction::Left))),
            Key::Right | Key::Char('l') => Ok(Event::Msg(Msg::ChangeDirection(Direction::Right))),
            _ => Ok(Event::NoOp),
        },
        Event::Msg(msg) => match msg {
            Msg::ChangeDirection(direction) => match &model.snake.direction == direction {
                true => {
                    model.fast = true;
                    Ok(Event::NoOp)
                }
                _ => match direction == &model.snake.direction.opposite() {
                    true => Ok(Event::NoOp),
                    _ => {
                        model.snake.direction = *direction;
                        Ok(Event::NoOp)
                    }
                },
            },
            Msg::FoodEaten => {
                model.food = Food::new(canvas)?;
                Ok(Event::NoOp)
            }
            Msg::GameOver => {
                println!("\nGame Over!");
                Ok(Event::Stop)
            }
        },
        Event::NoOp => {
            let &(x, y) = model.snake.body.front().unwrap();
            {
                let next = match model.snake.direction {
                    Direction::Up => (x + 0, y - 1),
                    Direction::Down => (x + 0, y + 1),
                    Direction::Right => (x + 1, y + 0),
                    Direction::Left => (x - 1, y + 0),
                };

                if canvas.is_boundary(&next)? || model.snake.body.contains(&next) {
                    Ok(Event::Msg(Msg::GameOver))
                } else if next == model.food.position {
                    model.snake.body.push_front(next);
                    Ok(Event::Msg(Msg::FoodEaten))
                } else {
                    model.snake.body.push_front(next);
                    model.snake.body.pop_back();
                    if !model.fast {
                        std::thread::sleep(Duration::from_millis(150));
                    } else {
                        model.fast = false
                    }
                    Ok(Event::NoOp)
                }
            }
        }
        _ => Ok(Event::NoOp),
    }
}

fn view(canvas: &MyCanvas, model: &MyModel, position: &Position) -> io::Result<Option<TermPixel>> {
    match simple_border(canvas, model, position) {
        Ok(None) => match model.snake.body.contains(position) {
            true => match position == model.snake.body.front().unwrap() {
                true => Ok(Some((model.snake.head_ascii, model.snake.head_style))),
                _ => Ok(Some((model.snake.body_ascii, model.snake.body_style))),
            },
            _ => match position == &model.food.position {
                true => Ok(Some((model.food.ascii, model.food.style))),
                _ => Ok(Some((canvas.fill_ascii, canvas.fill_style))),
            },
        },
        border => border,
    }
}

fn main() {
    app::run(&init, &view, &update, None).unwrap();
}
