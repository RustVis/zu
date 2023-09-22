// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SegmentOutlined)]
pub fn segment_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SegmentOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,18h12v-2H9V18z M3,6v2h18V6H3z M9,13h12v-2H9V13z"/>
        </SvgIcon>
    }
}
