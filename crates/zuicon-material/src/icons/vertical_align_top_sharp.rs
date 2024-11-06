// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VerticalAlignTopSharp)]
pub fn vertical_align_top_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VerticalAlignTopSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8 11h3v10h2V11h3l-4-4-4 4zM4 3v2h16V3H4z"/>
        </SvgIcon>
    }
}
