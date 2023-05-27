// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(CheckBoxOutlineBlank)]
pub fn check_box_outline_blank(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("CheckBoxOutlineBlank"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z" />
        </SvgIcon>
    }
}