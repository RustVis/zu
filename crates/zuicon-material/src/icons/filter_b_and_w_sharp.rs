// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilterBAndWSharp)]
pub fn filter_b_and_w_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilterBAndWSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 3H3v18h18V3zm-2 16l-7-8v8H5l7-8V5h7v14z"/>
        </SvgIcon>
    }
}
