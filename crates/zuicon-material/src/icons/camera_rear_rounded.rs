// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraRearRounded)]
pub fn camera_rear_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraRearRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M10.85 18.85c-.31-.31-.85-.09-.85.36V20H6c-.55 0-1 .45-1 1s.45 1 1 1h4v.79c0 .45.54.67.85.35l1.79-1.79c.2-.2.2-.51 0-.71l-1.79-1.79zM18 20h-3c-.55 0-1 .45-1 1s.45 1 1 1h3c.55 0 1-.45 1-1s-.45-1-1-1zM17 0H7C5.9 0 5 .9 5 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V2c0-1.1-.9-2-2-2zm-5 6c-1.11 0-2-.9-2-2s.89-2 1.99-2 2 .9 2 2C14 5.1 13.1 6 12 6z"/>
        </SvgIcon>
    }
}
