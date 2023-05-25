// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Looks4Outlined)]
pub fn looks_4_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Looks4Outlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M.04 0h24v24h-24V0z" fill="none"/><path d="M19.04 3h-14c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16h-14V5h14v14zm-6-2h2V7h-2v4h-2V7h-2v6h4z"/>
        </SvgIcon>
    }
}
