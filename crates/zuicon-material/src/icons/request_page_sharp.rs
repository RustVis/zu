// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RequestPageSharp)]
pub fn request_page_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RequestPageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4.01L4,22h16V8L14,2z M15,11h-4v1h4v5h-2v1h-2v-1H9v-2h4v-1H9V9h2V8h2v1h2V11z"/>
        </SvgIcon>
    }
}
