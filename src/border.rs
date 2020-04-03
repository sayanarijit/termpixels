use crate::{Location, Object};

pub enum BorderType {
    Simple,
}

pub fn ascii_for_border_or(
    object: &dyn Object,
    location: &Location,
    border_type: BorderType,
    default: impl Fn() -> Option<char>,
) -> Option<char> {
    match border_type {
        BorderType::Simple => {
            if object.is_top_left_corner(location) {
                Some('┌')
            } else if object.is_top_right_corner(location) {
                Some('┐')
            } else if object.is_bottom_left_corner(location) {
                Some('└')
            } else if object.is_bottom_right_corner(location) {
                Some('┘')
            } else if object.is_right_boundary(location) || object.is_left_boundary(location) {
                Some('│')
            } else if object.is_top_boundary(location) || object.is_bottom_boundary(location) {
                Some('─')
            } else {
                default()
            }
        }
    }
}
