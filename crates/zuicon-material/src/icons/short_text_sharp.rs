// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShortTextSharp)]
pub fn short_text_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShortTextSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4 9h16v2H4V9zm0 4h10v2H4v-2z"/>
        </SvgIcon>
    }
}
