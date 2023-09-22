// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(ArrowDownward)]
pub fn arrow_downward(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("ArrowDownward"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z" />
        </SvgIcon>
    }
}
