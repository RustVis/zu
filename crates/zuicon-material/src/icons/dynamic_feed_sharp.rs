// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DynamicFeedSharp)]
pub fn dynamic_feed_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DynamicFeedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,3H10v10h12V3z M20,11h-8V7h8V11z"/><path d="M22,3H10v10h12V3z M20,11h-8V7h8V11z"/>
        </SvgIcon>
    }
}
