// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoStableSharp)]
pub fn video_stable_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoStableSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M4,18V6h2.95l-2.33,8.73L16.82,18H4z M20,18h-2.95l2.34-8.73L7.18,6H20V18z"/>
        </SvgIcon>
    }
}
