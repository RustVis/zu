// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::paper::Paper;

use crate::components::demo_box::DemoBox;

#[function_component(PaperPage)]
pub fn paper_page() -> Html {
    let style = "width: 128px; height: 128px; margin: 8px";

    html! {
        <div class="container">
        <h1>{"Paper"}</h1>
        <p>{"The background of an application resembles the flat, opaque texture of a sheet of paper"}</p>

        <h2>{"Basic paper"}</h2>
        <DemoBox>
            <Paper style={style} elevation={0} />
            <Paper style={style} />
            <Paper style={style} elevation={3} />
        </DemoBox>

        </div>
    }
}
