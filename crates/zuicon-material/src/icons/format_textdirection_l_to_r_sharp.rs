// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatTextdirectionLToRSharp)]
pub fn format_textdirection_l_to_r_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatTextdirectionLToRSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9 10v5h2V4h2v11h2V4h2V2H9C6.79 2 5 3.79 5 6s1.79 4 4 4zm12 8l-4-4v3H5v2h12v3l4-4z"/>
        </SvgIcon>
    }
}
