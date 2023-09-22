// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Segment)]
pub fn segment(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Segment"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M9 18h12v-2H9v2zM3 6v2h18V6H3zm6 7h12v-2H9v2z"/>
        </SvgIcon>
    }
}
