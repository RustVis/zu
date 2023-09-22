// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PublishTwoTone)]
pub fn publish_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PublishTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M9.83 12H11v6h2v-6h1.17L12 9.83z" opacity=".3"/><path d="M5 4h14v2H5zm7 3l-7 7h4v6h6v-6h4l-7-7zm1 5v6h-2v-6H9.83L12 9.83 14.17 12H13z"/>
        </SvgIcon>
    }
}
