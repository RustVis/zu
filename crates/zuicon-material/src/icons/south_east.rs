// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SouthEast)]
pub fn south_east(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SouthEast"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9h-2v6.59L5.41,4L4,5.41L15.59,17H9v2h10V9z"/>
        </SvgIcon>
    }
}
