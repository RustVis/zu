// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RequestQuoteSharp)]
pub fn request_quote_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RequestQuoteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4v20h16V8L14,2z M15,12h-4v1h4v5h-2v1h-2v-1H9v-2h4v-1H9v-5h2V9h2v1h2V12z M13,8V3.5L17.5,8H13z"/>
        </SvgIcon>
    }
}
