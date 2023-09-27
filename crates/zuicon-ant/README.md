
# About
zuicon-ant is based on [Ant Design SVG Icons](https://github.com/ant-design/ant-design-icons).


## How to use

First add this to `Cargo.toml`:
```toml
[dependencies.zuicon-ant]
version = "0.2"
features = [
  "Home",
  "Mail",
]
```

Now `Home` and `Mail` are available.

Then import specific icons in yew components:
`use zuicon_ant::filled::{Home, Mail};`


## License
This library is release in [LGPL-3.0](LICENSE).
