// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbIridescentSharp)]
pub fn wb_iridescent_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WbIridescentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5 15h14V9.05H5V15zm6-14v3h2V1h-2zm8.04 2.6l-1.79 1.79 1.41 1.41 1.8-1.79-1.42-1.41zM13 23v-2.95h-2V23h2zm7.45-3.91l-1.8-1.79-1.41 1.41 1.79 1.8 1.42-1.42zM3.55 5.01L5.34 6.8l1.41-1.41L4.96 3.6 3.55 5.01zM4.96 20.5l1.79-1.8-1.41-1.41-1.79 1.79 1.41 1.42z"/>
        </SvgIcon>
    }
}
