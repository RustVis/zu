// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlaylistAddCircleRounded)]
pub fn playlist_add_circle_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlaylistAddCircleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M7,8c0-0.55,0.45-1,1-1h5c0.55,0,1,0.45,1,1v0 c0,0.55-0.45,1-1,1H8C7.45,9,7,8.55,7,8L7,8z M10,14c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h1 C9.55,13,10,13.45,10,14L10,14z M8,12c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h5c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H8z M18,15 h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1s1,0.45,1,1v1h1c0.55,0,1,0.45,1,1 S18.55,15,18,15z"/>
        </SvgIcon>
    }
}
