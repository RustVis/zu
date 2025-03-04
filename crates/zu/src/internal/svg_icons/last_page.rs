// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(LastPage)]
pub fn last_page(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("LastPage"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5.59 7.41L10.18 12l-4.59 4.59L7 18l6-6-6-6zM16 6h2v12h-2z" />
        </SvgIcon>
    }
}
