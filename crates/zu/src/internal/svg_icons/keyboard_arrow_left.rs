// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardArrowLeft)]
pub fn keyboard_arrow_left(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("KeyboardArrowLeft"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.41 16.09l-4.58-4.59 4.58-4.59L14 5.5l-6 6 6 6z" />
        </SvgIcon>
    }
}
