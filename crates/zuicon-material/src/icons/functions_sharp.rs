// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FunctionsSharp)]
pub fn functions_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FunctionsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18 4H6v2l6.5 6L6 18v2h12v-3h-7l5-5-5-5h7V4z"/>
        </SvgIcon>
    }
}
