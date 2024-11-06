// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatQuoteSharp)]
pub fn format_quote_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatQuoteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5 17h3l2-4V7H4v6h3l-2 4zm10 0h3l2-4V7h-6v6h3l-2 4z"/>
        </SvgIcon>
    }
}
