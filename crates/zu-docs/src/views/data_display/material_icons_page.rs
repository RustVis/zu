// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, Html};
use zu::code::Code;

#[function_component(MaterialIconsPage)]
pub fn material_icons_page() -> Html {
    // TODO(Shaohua): Add icon list.

    html! {
        <div class="container">
        <h1>{"Material Icons"}</h1>
        <p>{"zuicon-material includes the 2,100+ official Material Icons converted to SvgIcon components.
         Use the following command to install it:"}</p>
        <div class="code-root">
            <Code>
                {"cargo add zuicon-material"}
            </Code>
        </div>

        </div>
    }
}
