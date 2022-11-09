# Flocaliza XML

```rust
use flocariza_macros::localizable;
use serde_derive::{Deserialize, Serialize};
#[localizable]
#[derive(Deserialize, Serialize)]
struct App {
    pub name: Localized<String>,
    pub description: Localized<MarkupText>,
}

#[derive(Deserialize, Serialize)]
pub struct MarkupText {
    pub string: String,
}
fn main() {}
```

## Rust complains Localized<> type does not exist
Make sure `#[localizable]` is before the derives.

## License
MIT OR APACHE-2.0