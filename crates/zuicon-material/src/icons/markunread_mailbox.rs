// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkunreadMailbox)]
pub fn markunread_mailbox(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkunreadMailbox"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M-618-3000H782V600H-618zM0 0h24v24H0z" fill="none"/><path d="M20 6H10v6H8V4h6V0H6v6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/>
        </SvgIcon>
    }
}
