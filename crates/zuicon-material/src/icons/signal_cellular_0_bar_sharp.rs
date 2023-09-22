// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellular0BarSharp)]
pub fn signal_cellular_0_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellular0BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M2,22h20V2L2,22z M20,20H6.83L20,6.83V20z"/>
        </SvgIcon>
    }
}
