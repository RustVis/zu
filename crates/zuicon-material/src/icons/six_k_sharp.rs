// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixKSharp)]
pub fn six_k_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SixKSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,12.5h1.5V14H8V12.5z M21,3H3v18h18V3z M11,10.5H8v1h3V15H6.5V9H11V10.5z M18,15h-1.75l-1.75-2.25V15H13V9h1.5v2.25 L16.25,9H18l-2.25,3L18,15z"/>
        </SvgIcon>
    }
}
