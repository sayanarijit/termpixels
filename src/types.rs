pub use crate::canvas::Canvas;
pub use crate::event::Event;
use ansi_term::Style;
use std::io;

pub type Position = (u16, u16); // x, y
pub type Size = (u16, u16); // width, height
pub type TermPixel = (char, Style);

pub trait Model: Sized {}
impl<T: Sized> Model for T {}

pub trait Init<C: Canvas, M: Model>: Fn() -> io::Result<(C, M)> {}
impl<T, C, M> Init<C, M> for T
where
    C: Canvas,
    M: Model,
    T: Fn() -> io::Result<(C, M)>,
{
}

pub trait View<C: Canvas, M: Model>:
    Fn(&C, &M, &Position) -> io::Result<Option<TermPixel>>
{
}

impl<T, C, M> View<C, M> for T
where
    C: Canvas,
    M: Model,
    T: Fn(&C, &M, &Position) -> io::Result<Option<TermPixel>>,
{
}

pub trait Update<C: Canvas, M: Model, E>:
    Fn(&C, &mut M, &Event<E>) -> io::Result<Event<E>>
{
}
impl<T, C, M, E> Update<C, M, E> for T
where
    C: Canvas,
    M: Model,
    T: Fn(&C, &mut M, &Event<E>) -> io::Result<Event<E>>,
{
}
