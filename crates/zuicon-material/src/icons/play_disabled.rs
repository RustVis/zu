// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlayDisabled)]
pub fn play_disabled(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlayDisabled"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0zm0 0h24v24H0V0zm11.75 11.47l-.11-.11.11.11z" fill="none"/><path d="M8 5.19V5l11 7-2.55 1.63L8 5.19zm12 14.54l-5.11-5.11L8 7.73 4.27 4 3 5.27l5 5V19l5.33-3.4 5.4 5.4L20 19.73z"/>
        </SvgIcon>
    }
}
