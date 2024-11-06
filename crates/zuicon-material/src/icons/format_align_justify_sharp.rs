// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatAlignJustifySharp)]
pub fn format_align_justify_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatAlignJustifySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3 21h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18V7H3v2zm0-6v2h18V3H3z"/>
        </SvgIcon>
    }
}
