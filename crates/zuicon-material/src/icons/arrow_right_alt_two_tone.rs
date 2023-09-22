// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowRightAltTwoTone)]
pub fn arrow_right_alt_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowRightAltTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M16.01 11H4v2h12.01v3L20 12l-3.99-4v3z"/>
        </SvgIcon>
    }
}
