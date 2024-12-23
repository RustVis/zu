// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignHorizontalRight)]
pub fn align_horizontal_right(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignHorizontalRight"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2h2v20h-2V2z M2,10h16V7H2V10z M8,17h10v-3H8V17z"/>
        </SvgIcon>
    }
}
