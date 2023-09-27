
# About
Wrapper of bootstrap icons for yew framework.


## How to use

First add this to `Cargo.toml`:
```toml
[dependencies.zuicon-bs]
version = "0.2"
features = [
  "Mailbox",
  "Map",
]
```

Now  `Mailbox` and `Map` are available.

Then import specific icons in yew components:
`use zuicon_bs::{Mailbox, Map};`


## License
This library is release in [LGPL-3.0](LICENSE).
