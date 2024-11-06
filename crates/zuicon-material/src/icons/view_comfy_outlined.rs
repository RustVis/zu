// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyOutlined)]
pub fn view_comfy_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewComfyOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M4,6h16v5H4V6z M4,18v-5h4v5H4z M10,18v-5h10v5H10z"/>
        </SvgIcon>
    }
}
