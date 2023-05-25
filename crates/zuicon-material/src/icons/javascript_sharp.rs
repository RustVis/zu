// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(JavascriptSharp)]
pub fn javascript_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("JavascriptSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,15v-2h1.5v0.5h2v-1H12V9h5v2h-1.5v-0.5h-2v1H17V15H12z M9,9v4.5H7.5v-1H6V15h4.5V9H9z"/>
        </SvgIcon>
    }
}
