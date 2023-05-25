// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlayArrowTwoTone)]
pub fn play_arrow_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlayArrowTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M10 8.64v6.72L15.27 12z" opacity=".3"/><path d="M8 19l11-7L8 5v14zm2-10.36L15.27 12 10 15.36V8.64z"/>
        </SvgIcon>
    }
}
