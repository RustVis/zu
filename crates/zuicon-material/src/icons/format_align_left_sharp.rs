// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatAlignLeftSharp)]
pub fn format_align_left_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatAlignLeftSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15 15H3v2h12v-2zm0-8H3v2h12V7zM3 13h18v-2H3v2zm0 8h18v-2H3v2zM3 3v2h18V3H3z"/>
        </SvgIcon>
    }
}
