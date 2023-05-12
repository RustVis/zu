// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use stylist::style;
use yew::{function_component, html, Html};
use zu::r#box::Box;

#[function_component(BoxPage)]
pub fn box_page() -> Html {
    let style = style!(
        r#"
        width: 300px;
        height: 300px;
        background-color: var(--zu-palette-primary-dark);
        &:hover {
            background-color: var(--zu-palette-primary-main);
            opacity: 0.9;
        }
    "#
    )
    .unwrap();

    html! {
        <div class="container">
        <h1>{"Box"}</h1>
        <div class="demo-box">
            <Box classes={style.get_class_name().to_owned()}></Box>
        </div>

        <h2>{"Override component"}</h2>
        <div class="demo-box">
            <Box component="span" style="border: 1px dashed grey; padding: 16px">{"Save"}</Box>
        </div>
        </div>
    }
}
