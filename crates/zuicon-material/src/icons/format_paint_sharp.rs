// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatPaintSharp)]
pub fn format_paint_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatPaintSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18 4V2H4v6h14V6h1v4H9v12h4V12h8V4h-3z"/>
        </SvgIcon>
    }
}
