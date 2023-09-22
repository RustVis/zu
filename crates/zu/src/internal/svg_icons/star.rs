// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(Star)]
pub fn star(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("Star"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z" />
        </SvgIcon>
    }
}
