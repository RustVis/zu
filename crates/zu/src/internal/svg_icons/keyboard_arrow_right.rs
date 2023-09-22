// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardArrowRight)]
pub fn keyboard_arrow_right(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("KeyboardArrowRight"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.59 16.34l4.58-4.59-4.58-4.59L10 5.75l6 6-6 6z" />
        </SvgIcon>
    }
}
