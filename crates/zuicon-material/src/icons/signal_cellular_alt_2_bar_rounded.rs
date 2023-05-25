// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularAlt2BarRounded)]
pub fn signal_cellular_alt_2_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularAlt2BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.5,20C5.67,20,5,19.33,5,18.5v-3C5,14.67,5.67,14,6.5,14S8,14.67,8,15.5v3C8,19.33,7.33,20,6.5,20z M12.5,20 c-0.83,0-1.5-0.67-1.5-1.5v-8C11,9.67,11.67,9,12.5,9S14,9.67,14,10.5v8C14,19.33,13.33,20,12.5,20z"/>
        </SvgIcon>
    }
}
