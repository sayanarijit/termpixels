extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Object)]
pub fn object_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Object for #name {
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

#[proc_macro_derive(View)]
pub fn view_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl View for #name {
            fn ascii_for(&self, _location: &Location) -> Option<char> {
                Some(' ')
            }
            fn style_for(&self, _location: &Location) -> ansi_term::Style {
                ansi_term::Style::default()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Update)]
pub fn update_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Update for #name {
            fn update(&mut self) -> io::Result<()> {
                Ok(())
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
                location: &Location,
            ) -> io::Result<()> {
                if let Some(ch) = self.ascii_for(location) {
                    let style = self.style_for(location);
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Goto(location.0, location.1),
                        style.paint(ch.to_string())
                    )
                } else {
                    Ok(())
                }
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
                location: &Location,
            ) -> std::io::Result<()> {
                if self.ascii_for(location).is_some() {
                    write!(stdout, "{} ", termion::cursor::Goto(location.0, location.1))
                } else {
                    Ok(())
                }
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
                refresh_interval: Option<std::time::Duration>,
            ) -> std::io::Result<()> {
                loop {
                    self.update()?;
                    self.paint_all(stdout)?;

                    if let Some(result) = events.next() {
                        let event = result?;
                        if let Err(err) = self.on_event(event) {
                            match err.kind() {
                                std::io::ErrorKind::Interrupted => {
                                    break Ok(());
                                }
                                _ => {
                                    break Err(err);
                                }
                            }
                        }
                    };

                    if let Some(interval) = refresh_interval {
                        std::thread::sleep(interval);
                    }
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
