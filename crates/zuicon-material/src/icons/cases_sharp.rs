// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CasesSharp)]
pub fn cases_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CasesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,5V1h-8v4H5v13h18V5H18z M16,5h-4V3h4V5z M3,9H1v13h18v-2H3V9z"/>
        </SvgIcon>
    }
}
