// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutboxSharp)]
pub fn outbox_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutboxSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M19,14h-4.18c-0.41,1.16-1.51,2-2.82,2s-2.4-0.84-2.82-2H5V5h14V14z"/>
        </SvgIcon>
    }
}
