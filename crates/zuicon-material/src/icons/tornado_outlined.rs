// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TornadoOutlined)]
pub fn tornado_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TornadoOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23,3H1l11,19L23,3z M19.53,5l-1.74,3H6.21L4.47,5H19.53z M10.26,15h3.48L12,18.01L10.26,15z M14.9,13H9.1l-1.74-3h9.27 L14.9,13z"/>
        </SvgIcon>
    }
}
