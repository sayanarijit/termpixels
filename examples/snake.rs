use ansi_term::{Color, Style};
use std::collections::VecDeque;
use std::io;
use std::time::Duration;
use std::time::SystemTime;
use termion::async_stdin;
use termion::event::{Event, Key};
use termion::terminal_size;
use termpixels::border::{ascii_for_border_or, BorderType};
use termpixels::prelude::*;
use termpixels_derive::{Clear, Object, Paint, Render};

#[derive(Copy, Clone, PartialEq)]
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

struct Snake {
    direction: Direction,
    body: VecDeque<Location>,
    style: Style,
    fast: bool,
}

impl Snake {
    fn is_body(&self, location: &Location) -> bool {
        self.body.contains(location)
    }

    fn set_direction_if_possible(&mut self, direction: Direction) -> bool {
        if direction == self.direction {
            self.fast = true;
            true
        } else if direction == self.direction.opposite() {
            false
        } else {
            self.direction = direction;
            true
        }
    }

    fn move_forward(&mut self) -> Result<Location, &'static str> {
        let (x, y) = self.body.front().unwrap();
        let next = match self.direction {
            Direction::Up => (x + 0, y - 1),
            Direction::Down => (x + 0, y + 1),
            Direction::Right => (x + 1, y + 0),
            Direction::Left => (x - 1, y + 0),
        };

        if self.is_body(&next) {
            return Err("game over.");
        };

        self.body.push_front(next);
        Ok(self.body.pop_back().unwrap())
    }

    fn eat(&mut self, tail: Location) -> Result<(), &'static str> {
        self.body.push_back(tail);
        Ok(())
    }
}

struct Food {
    location: Location,
    style: Style,
    ascii: char,
}

impl Food {
    fn new() -> Food {
        let bad_rand1 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let bad_rand2 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();

        let (w, h) = terminal_size().unwrap();

        Food {
            location: (
                (bad_rand1 as u16 % (w - 2) + 1),
                (bad_rand2 as u16 % (h - 2) + 1),
            ),
            style: Style::default().fg(Color::Green),
            ascii: '❄',
        }
    }
}

#[derive(Object, Paint, Render, Clear)]
struct Game {
    position: Location,
    size: Size,
    snake: Snake,
    background_style: Style,
    food: Food,
    game_over: bool,
}

impl View for Game {
    fn style_for(&self, location: &Location) -> Style {
        if location == &self.food.location {
            self.food.style
        } else if self.snake.is_body(location) {
            self.snake.style
        } else {
            self.background_style
        }
    }

    fn ascii_for(&self, location: &Location) -> Option<char> {
        ascii_for_border_or(self, location, BorderType::Simple, || {
            if location == &self.food.location {
                Some(self.food.ascii)
            } else if self.snake.is_body(location) {
                Some('⬤')
            } else {
                Some(' ')
            }
        })
    }
}

impl Update for Game {
    fn update(&mut self) -> io::Result<()> {
        if let Ok(tail) = self.snake.move_forward() {
            if self.snake.is_body(&self.food.location) {
                self.snake.eat(tail).unwrap();
                self.food = Food::new();
            };

            if self.is_boundary(self.snake.body.front().unwrap()) {
                self.game_over = true;
            };
        } else {
            self.game_over = true;
        };

        if self.game_over {
            eprintln!("Game Over!");
            std::process::exit(1)
        };

        if self.snake.fast {
            self.snake.fast = false;
        } else {
            std::thread::sleep(Duration::from_millis(200));
        }

        Ok(())
    }
}

impl EventHandler for Game {
    fn on_event(&mut self, event: termion::event::Event) -> io::Result<()> {
        match event {
            Event::Key(key) => match key {
                Key::Up => {
                    self.snake.set_direction_if_possible(Direction::Up);
                    Ok(())
                }
                Key::Down => {
                    self.snake.set_direction_if_possible(Direction::Down);
                    Ok(())
                }
                Key::Left => {
                    self.snake.set_direction_if_possible(Direction::Left);
                    Ok(())
                }
                Key::Right => {
                    self.snake.set_direction_if_possible(Direction::Right);
                    Ok(())
                }
                Key::Ctrl('c') => Err(io::Error::from(io::ErrorKind::Interrupted)),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

fn main() {
    let (width, height) = terminal_size().unwrap();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut game = Game {
        position: (1, 1),
        food: Food::new(),
        size: (width, height),
        background_style: Style::default().on(Color::Black),
        snake: Snake {
            direction: Direction::Up,
            body: VecDeque::new(),
            style: Style::default().fg(Color::Cyan),
            fast: false,
        },
        game_over: false,
    };

    game.snake.body.push_back((width / 2, height / 2));
    game.snake.body.push_back((width / 2, height / 2 + 1));
    game.snake.body.push_back((width / 2, height / 2 + 2));

    let stdin = async_stdin();
    let mut events = stdin.events();

    if let Err(err) = game.render(&mut stdout, &mut events, Some(Duration::from_millis(50))) {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
