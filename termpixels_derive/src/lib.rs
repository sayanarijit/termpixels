extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TermObject)]
pub fn term_object_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl TermObject for #name {
            fn position(&self) -> Location {
                self.position
            }
            fn set_position(&mut self, location: &Location) {
                self.position.0 = location.0;
                self.position.1 = location.1;
            }
            fn size(&self) -> Size {
                self.size
            }
            fn set_size(&mut self, size: &Location) {
                self.size.0 = size.0;
                self.size.1 = size.1;
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Shape)]
pub fn shape_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Shape for #name {
            fn ascii_for(&self, _location: &Location) -> char {
                ' '
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Appearance)]
pub fn appearance_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Appearance for #name {
            fn style_for(&self, _location: &Location) -> ansi_term::Style {
                ansi_term::Style::default()
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(EventHandler)]
pub fn event_handler_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl EventHandler for #name {
            fn on_event(&mut self, event: termion::event::Event) -> io::Result<()> {
                match event {
                    termion::event::Event::Key(termion::event::Key::Ctrl('c')) => {
                        Err(std::io::Error::from(std::io::ErrorKind::Interrupted))
                    }
                    _ => Ok(()),
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Paint)]
pub fn paint_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
       impl Paint for #name {
           fn paint(
               &self,
               stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
               location: Location,
           ) -> io::Result<()> {
               let ch = self.ascii_for(&location);
               let style = self.style_for(&location);

               write!(
                   stdout,
                   "{}{}",
                   termion::cursor::Goto(location.0, location.1),
                   style.paint(ch.to_string())
               )
           }
       }
    };
    gen.into()
}

#[proc_macro_derive(Clear)]
pub fn clear_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Clear for #name {
            fn clear(
                &self,
                stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
                location: Location,
            ) -> std::io::Result<()> {
                write!(stdout, "{} ", termion::cursor::Goto(location.0, location.1))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Render)]
pub fn render_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Render for #name {
            fn render<R: std::io::Read>(
                &mut self,
                stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
                events: &mut termion::input::Events<R>,
            ) -> std::io::Result<()> {
                loop {
                    self.paint_all(stdout)?;
                    if let Some(result) = events.next() {
                        let event = result?;
                        if let Err(err) = self.on_event(event) {
                            match err.kind() {
                                std::io::ErrorKind::Interrupted => {
                                    break self.clear_all(stdout);
                                }
                                _ => {
                                    self.clear_all(stdout)?;
                                    break Err(err);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BorderedShape)]
pub fn pordered_shape_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Shape for #name {
            fn ascii_for(&self, location: &Location) -> char {
                if self.is_top_left_corner(location) {
                    '┌'
                } else if self.is_top_right_corner(location) {
                    '┐'
                } else if self.is_bottom_left_corner(location) {
                    '└'
                } else if self.is_bottom_right_corner(location) {
                    '┘'
                } else if self.is_right_boundary(location) || self.is_left_boundary(location) {
                    '│'
                } else if self.is_top_boundary(location) || self.is_bottom_boundary(location) {
                    '─'
                } else {
                    ' '
                }
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
