// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VerticalAlignTopTwoTone)]
pub fn vertical_align_top_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VerticalAlignTopTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M4 3h16v2H4zm4 8h3v10h2V11h3l-4-4z"/>
        </SvgIcon>
    }
}
