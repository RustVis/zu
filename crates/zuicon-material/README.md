
# About
Wrapper of material design icons for yew framework.

Material icons are placed in different themes:
1. baseline (default)
2. outline
3. round
4. twotone
5. sharp

## How to use
First add this to `Cargo.toml`:
```toml
[dependencies.zuicon-material]
version = "0.2"
features = [
  "Home",
  "Email",
]
```

`Home` and `Email` are available.

Then import specific icons in yew components:
`use zuicon_material::{email::Email, home::Home};`

## Related projects
- [MUI Material Icons][icons-material]

[icons-material]: https://github.com/mui/material-ui/tree/master/packages/mui-icons-material


## License
This library is release in [Apache License](LICENSE).
