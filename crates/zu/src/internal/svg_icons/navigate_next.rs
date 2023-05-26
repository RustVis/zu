// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(NavigateNext)]
pub fn navigate_next(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("NavigateNext"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" />
        </SvgIcon>
    }
}
