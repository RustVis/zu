// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PolylineOutlined)]
pub fn polyline_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PolylineOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,16v1.26l-6-3v-3.17L11.7,8H16V2h-6v4.9L7.3,10H3v6h5l7,3.5V22h6v-6H15z M12,4h2v2h-2V4z M7,14H5v-2h2V14z M19,20h-2v-2 h2V20z"/>
        </SvgIcon>
    }
}
