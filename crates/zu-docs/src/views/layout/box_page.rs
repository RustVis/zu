// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use stylist::Style;
use yew::{classes, function_component, html, Html};
use zu::r#box::Box;

use crate::components::demo_box::DemoBox;

#[function_component(BoxPage)]
pub fn box_page() -> Html {
    let style = Style::new(
        r#"
        width: 300px;
        height: 300px;
        background-color: var(--zu-palette-primary-dark);
        &:hover {
            background-color: var(--zu-palette-primary-main);
            opacity: 0.9;
        }
    "#,
    )
    .unwrap();

    html! {
        <div class="container">
        <h1>{"Box"}</h1>
        <DemoBox>
            <Box classes={classes!(style.get_class_name().to_string())}></Box>
        </DemoBox>

        <h2>{"Override component"}</h2>
        <DemoBox>
            <Box component="span" style="border: 1px dashed grey; padding: 16px">{"Save"}</Box>
        </DemoBox>
        </div>
    }
}
