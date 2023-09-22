// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DiamondTwoTone)]
pub fn diamond_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DiamondTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5L2,9l10,12L22,9L19,3z M17.76,5l1.5,3h-2.65l-1.5-3H17.76z M6.24,5h2.65l-1.5,3H4.74L6.24,5z M11,16.68L5.44,10H11 V16.68z M9.62,8l1.5-3h1.76l1.5,3H9.62z M13,16.68V10h5.56L13,16.68z"/>
        </SvgIcon>
    }
}
