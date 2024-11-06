// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoSettingsSharp)]
pub fn video_settings_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoSettingsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.72,17.57l1.23-0.98l-1.25-2.17L21.23,15c-0.23-0.17-0.48-0.31-0.75-0.42L20.25,13h-2.5l-0.24,1.58 c-0.26,0.11-0.51,0.26-0.74,0.42l-1.48-0.58l-1.25,2.17l1.24,0.99c-0.03,0.29-0.04,0.58-0.01,0.86l-1.23,0.98l1.25,2.17L16.77,21 c0.23,0.17,0.48,0.31,0.75,0.42L17.75,23h2.5l0.24-1.58c0.26-0.11,0.51-0.26,0.74-0.42l1.48,0.58l1.25-2.17l-1.24-0.99 C22.75,18.14,22.75,17.85,22.72,17.57z M19,19.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S19.83,19.5,19,19.5z"/>
        </SvgIcon>
    }
}
