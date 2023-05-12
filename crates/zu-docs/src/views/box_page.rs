// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::r#box::Box;

#[function_component(BoxPage)]
pub fn box_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Box"}</h1>

        <h2>{"Override component"}</h2>
        <div class="demo-box">
            <Box component="span" style="border: 1px dashed grey; padding: 16px">{"Save"}</Box>
        </div>
        </div>
    }
}
