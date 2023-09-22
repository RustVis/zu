// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(ArrowDropDown)]
pub fn arrow_drop_down(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("ArrowDropDown"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7 10l5 5 5-5z" />
        </SvgIcon>
    }
}
