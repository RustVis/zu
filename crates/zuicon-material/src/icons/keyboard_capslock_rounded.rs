// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardCapslockRounded)]
pub fn keyboard_capslock_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyboardCapslockRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12 8.41l3.89 3.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L12.71 6.3c-.39-.39-1.02-.39-1.41 0l-4.6 4.59c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 8.41zM7 18h10c.55 0 1-.45 1-1s-.45-1-1-1H7c-.55 0-1 .45-1 1s.45 1 1 1z"/>
        </SvgIcon>
    }
}
