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

#[proc_macro_derive(Paint)]
pub fn paint_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Paint for #name {
            fn paint_ascii_for(&self, _location: &Location) -> Option<char> {
                Some(' ')
            }
            fn paint_style_for(&self, _location: &Location) -> ansi_term::Style {
                ansi_term::Style::default().reverse()
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
            fn clear_ascii_for(&self, _location: &Location) -> Option<char> {
                Some(' ')
            }
            fn clear_style_for(&self, _location: &Location) -> ansi_term::Style {
                ansi_term::Style::default()
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
            fn view(&self) -> Vec<TermPixel> {
                let mut termpixels: Vec<TermPixel> = Vec::new();
                for y in self.top_boundary()..(self.bottom_boundary() + 1) {
                    for x in self.left_boundary()..(self.right_boundary() + 1) {
                        let location = (x, y);
                        if let Some(ascii) = self.paint_ascii_for(&location) {
                            let style = self.paint_style_for(&location);
                            termpixels.push((location, ascii, style));
                        } else if let Some(ascii) = self.clear_ascii_for(&location) {
                            let style = self.clear_style_for(&location);
                            termpixels.push((location, ascii, style));
                        }
                    }
                }
                termpixels
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
                        Err(io::Error::from(io::ErrorKind::Interrupted))
                    }
                    _ => Ok(()),
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
            fn render<R: Read>(
                &mut self,
                stdout: &mut termion::raw::RawTerminal<io::Stdout>,
                events: &mut termion::input::Events<R>,
                refresh_interval: Option<std::time::Duration>,
            ) -> io::Result<()> {
                let mut view: std::collections::HashMap<Location, (char, ansi_term::Style)> =
                    std::collections::HashMap::new();
                let mut updates = self.view();

                write!(stdout, "{}", termion::cursor::Hide)?;

                loop {
                    for (location, ascii, style) in updates.iter() {
                        write!(
                            stdout,
                            "{}{}",
                            termion::cursor::Goto(location.0, location.1),
                            style.paint(ascii.to_string())
                        )?;
                    }
                    stdout.flush()?;

                    if let Some(result) = events.next() {
                        let event = result?;
                        if let Err(err) = self.on_event(event) {
                            match err.kind() {
                                io::ErrorKind::Interrupted => {
                                    write!(stdout, "{}", termion::cursor::Show)?;
                                    break Ok(());
                                }
                                _ => {
                                    write!(stdout, "{}", termion::cursor::Show)?;
                                    break Err(err);
                                }
                            }
                        }
                    };

                    if let Some(interval) = refresh_interval {
                        std::thread::sleep(interval);
                    }

                    updates.clear();
                    self.update()?;

                    for (location, ascii, style) in self.view() {
                        if let Some((curr_ascii, curr_style)) = view.get(&location) {
                            if curr_ascii != &ascii || curr_style != &style {
                                updates.push((location, ascii, style));
                                view.insert(location, (ascii, style));
                            }
                        } else {
                            updates.push((location, ascii, style));
                            view.insert(location, (ascii, style));
                        }
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
