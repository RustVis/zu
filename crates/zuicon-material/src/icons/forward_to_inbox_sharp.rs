// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForwardToInboxSharp)]
pub fn forward_to_inbox_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForwardToInboxSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4H2v16h11v-2H4V8l8,5l8-5v5h2V4z M12,11L4,6h16L12,11z M19,15l4,4l-4,4v-3h-4v-2h4V15z"/>
        </SvgIcon>
    }
}
