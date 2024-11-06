// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditOffRounded)]
pub fn edit_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M2.1 3.51c-.39.39-.39 1.02 0 1.41l6.61 6.61-5.56 5.57c-.1.1-.15.22-.15.36v3.04c0 .28.22.5.5.5h3.04c.13 0 .26-.05.35-.15l5.56-5.56 6.61 6.61c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L3.52 3.51c-.4-.39-1.03-.39-1.42 0z"/><path d="M20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
        </SvgIcon>
    }
}
