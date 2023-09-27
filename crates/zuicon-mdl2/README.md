
# About
Migrate of Fluentui Icons (MDL2) to yew framework.


## How to use

First add this to `Cargo.toml`:
```toml
[dependencies.zuicon-mdl2]
version = "0.2"
features = [
  "Home",
  "PublicEmail",
]
```

Now  `Home` and `PublicEmail` are available.

Then import specific icons in yew components:
`use zuicon_mdl2::{Home, PublicEmail};`


## License
This library is release in [LGPL-3.0](LICENSE).
