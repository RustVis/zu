// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellular0Bar)]
pub fn signal_cellular_0_bar(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellular0Bar"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6.83V20H6.83L20,6.83 M22,2L2,22h20V2L22,2z"/>
        </SvgIcon>
    }
}
