// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LooksTwoRounded)]
pub fn looks_two_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LooksTwoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 8c0 1.1-.9 2-2 2h-2v2h3c.55 0 1 .45 1 1s-.45 1-1 1h-4c-.55 0-1-.45-1-1v-3c0-1.1.9-2 2-2h2V9h-3c-.55 0-1-.45-1-1s.45-1 1-1h3c1.1 0 2 .9 2 2v2z"/>
        </SvgIcon>
    }
}
