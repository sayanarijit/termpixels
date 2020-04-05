pub use termion::event::{Event as Input, Key, MouseEvent as Mouse};

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    NoOp,
    Input(Input),
    Msg(u32),
    GracefulStop,
    Stop,
}
