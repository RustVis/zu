// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddToQueueRounded)]
pub fn add_to_queue_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddToQueueRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v1c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-1h5c1.1 0 2-.9 2-2V5c0-1.11-.9-2-2-2zm-1 14H4c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v10c0 .55-.45 1-1 1zm-4-6c0 .55-.45 1-1 1h-2v2c0 .55-.45 1-1 1s-1-.45-1-1v-2H9c-.55 0-1-.45-1-1s.45-1 1-1h2V8c0-.55.45-1 1-1s1 .45 1 1v2h2c.55 0 1 .45 1 1z"/>
        </SvgIcon>
    }
}
