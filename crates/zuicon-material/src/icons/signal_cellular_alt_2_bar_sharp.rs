// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularAlt2BarSharp)]
pub fn signal_cellular_alt_2_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularAlt2BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,14h3v6H5V14z M11,9h3v11h-3V9z"/>
        </SvgIcon>
    }
}
