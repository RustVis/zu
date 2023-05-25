// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{function_component, Html, html};

#[function_component(MaterialIconPage)]
pub fn material_icon_page() -> Html {
    // TODO(Shaohua): Add icon list.

    html!{
        <div class="container">
        <h1>{"Material Icons"}</h1>
        <p>{"zuicon-material includes the 2,100+ official Material Icons converted to SvgIcon components.
         Use the following command to install it:"}</p>
        <div class="code-root">
            <pre>
                <code>{"cargo add zuicon-material"}</code>
            </pre>
        </div>
        </div>
    }
}