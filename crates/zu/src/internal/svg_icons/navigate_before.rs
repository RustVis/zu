// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(NavigateBefore)]
pub fn navigate_before(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("NavigateBefore"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z" />
        </SvgIcon>
    }
}
