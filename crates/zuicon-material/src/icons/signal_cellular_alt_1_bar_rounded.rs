// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularAlt1BarRounded)]
pub fn signal_cellular_alt_1_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularAlt1BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.5,20C5.67,20,5,19.33,5,18.5v-3C5,14.67,5.67,14,6.5,14S8,14.67,8,15.5v3C8,19.33,7.33,20,6.5,20z"/>
        </SvgIcon>
    }
}
