// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextRotationAngledownRounded)]
pub fn text_rotation_angledown_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextRotationAngledownRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M15 20.5v-2.54c0-.45-.54-.67-.85-.35l-.56.56L5.1 9.68c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l8.49 8.49-.56.56c-.32.32-.1.86.34.86h2.54c.28 0 .5-.23.5-.5zM11.25 8.48l3.54 3.54-.67 1.6c-.15.36-.07.77.21 1.05.49.49 1.31.32 1.57-.32l3.61-9.09c.17-.42.07-.91-.25-1.23-.32-.32-.8-.42-1.23-.25l-9.1 3.6c-.64.25-.81 1.08-.32 1.57.27.27.68.35 1.04.2l1.6-.67zm6.59-3.05l-2.23 4.87-2.64-2.64 4.87-2.23z"/>
        </SvgIcon>
    }
}
