// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HomeWorkTwoTone)]
pub fn home_work_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HomeWorkTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,11h2v2h-2v2h2v2h-2v2h4V5h-9v1.4l5,3.57V11z M17,7h2v2h-2V7z" opacity=".3"/><path d="M1,11v10h6v-5h2v5h6V11L8,6L1,11z M13,19h-2v-5H5v5H3v-7l5-3.5l5,3.5V19z"/>
        </SvgIcon>
    }
}
