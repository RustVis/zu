// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MusicVideoRounded)]
pub fn music_video_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MusicVideoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M20,18H4V6h16V18z"/><path d="M10.84,16.98c1.26-0.17,2.16-1.33,2.16-2.6L13,9h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v4.51 c-0.46-0.35-1.02-0.54-1.66-0.51c-1.11,0.07-2.09,0.92-2.3,2.02C7.73,15.73,9.15,17.2,10.84,16.98z"/>
        </SvgIcon>
    }
}
