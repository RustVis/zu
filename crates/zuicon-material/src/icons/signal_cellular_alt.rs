// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularAlt)]
pub fn signal_cellular_alt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularAlt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M17 4h3v16h-3zM5 14h3v6H5zm6-5h3v11h-3z"/>
        </SvgIcon>
    }
}
