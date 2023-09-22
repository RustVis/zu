// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatIndentDecreaseSharp)]
pub fn format_indent_decrease_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatIndentDecreaseSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11 17h10v-2H11v2zm-8-5l4 4V8l-4 4zm0 9h18v-2H3v2zM3 3v2h18V3H3zm8 6h10V7H11v2zm0 4h10v-2H11v2z"/>
        </SvgIcon>
    }
}
