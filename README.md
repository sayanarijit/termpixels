termpixels
==========
A work in progress TUI framework.

Add in Cargo.toml:

```toml
[dependencies]
termpixels = "*"

```

Example:

```bash
cargo run --release --example green_box
```

Code:

```rust
use std::io;
use termion::raw::IntoRawMode;
use termpixels::{Color, Location, Renderable, Size, Style};

struct GreenBox {
    ascii: char,
    size: Size,
    position: Location,
}

impl Renderable for GreenBox {
    fn size(&self) -> &Size {
        &self.size
    }

    fn position(&self) -> &Location {
        &self.position
    }
    fn ascii_for(&self, _: &Location) -> char {
        self.ascii
    }
    fn style_for(&self, _: &Location) -> Style {
        Style::default().on(Color::Green)
    }
}

fn main() {
    let panel = GreenBox {
        ascii: ' ',
        size: (10, 20),
        position: (2, 2),
    };
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    panel.render(&mut stdout).unwrap();
}
```
