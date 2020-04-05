use crate::canvas::Canvas;
use crate::types::*;
use ansi_term::Style;
use std::io;

pub fn simple_border<C: Canvas, M: Model>(
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
