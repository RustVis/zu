// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WidthNormalSharp)]
pub fn width_normal_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WidthNormalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4H2v16h20V4z M4,6h4v12H4V6z M20,18h-4V6h4V18z"/>
        </SvgIcon>
    }
}
