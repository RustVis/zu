// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlashOnRounded)]
pub fn flash_on_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlashOnRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 3v9c0 .55.45 1 1 1h2v7.15c0 .51.67.69.93.25l5.19-8.9c.39-.67-.09-1.5-.86-1.5H13l2.49-6.65c.25-.65-.23-1.35-.93-1.35H8c-.55 0-1 .45-1 1z"/>
        </SvgIcon>
    }
}
