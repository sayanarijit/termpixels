pub use termion::event::{Event as Input, Key, MouseEvent as Mouse};

#[derive(Debug, PartialEq, Clone)]
pub enum Event<T> {
    NoOp,
    Input(Input),
    Msg(T),
    GracefulStop,
    Stop,
}
