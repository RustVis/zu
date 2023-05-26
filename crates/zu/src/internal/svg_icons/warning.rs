// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(Warning)]
pub fn warning(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("Warning"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z" />
        </SvgIcon>
    }
}
