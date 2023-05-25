// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrAuto)]
pub fn hdr_auto(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrAuto"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M15.21,17l-0.98-2.81H9.78l-1,2.81h-1.9l4.13-11 h1.97l4.13,11H15.21z"/>
        </SvgIcon>
    }
}
