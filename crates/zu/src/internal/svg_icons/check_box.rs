// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(CheckBox)]
pub fn check_box(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("CheckBox"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
        </SvgIcon>
    }
}
