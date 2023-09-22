// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Woman)]
pub fn woman(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Woman"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.94,8.31C13.62,7.52,12.85,7,12,7s-1.62,0.52-1.94,1.31L7,16h3v6h4v-6h3L13.94,8.31z"/>
        </SvgIcon>
    }
}
