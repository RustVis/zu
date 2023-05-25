// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BungalowSharp)]
pub fn bungalow_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BungalowSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3L4.2,15.5l1.7,1.06L7,14.8V21h4v-5h2v5h4v-6.21l1.1,1.77l1.7-1.06L12,3z M13,14h-2v-2h2V14z"/>
        </SvgIcon>
    }
}
