// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VerticalAlignBottomSharp)]
pub fn vertical_align_bottom_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VerticalAlignBottomSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16 13h-3V3h-2v10H8l4 4 4-4zM4 19v2h16v-2H4z"/>
        </SvgIcon>
    }
}
