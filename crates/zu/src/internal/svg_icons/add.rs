// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(Add)]
pub fn add(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("Add"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
        </SvgIcon>
    }
}
