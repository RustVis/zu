// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(QueuePlayNextRounded)]
pub fn queue_play_next_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("QueuePlayNextRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v1c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-1h1c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v6c0 .55.45 1 1 1s1-.45 1-1V5c0-1.1-.9-2-2-2zm-8 7V8c0-.55-.45-1-1-1s-1 .45-1 1v2H9c-.55 0-1 .45-1 1s.45 1 1 1h2v2c0 .55.45 1 1 1s1-.45 1-1v-2h2c.55 0 1-.45 1-1s-.45-1-1-1h-2zm10.29 8.71l-3.04 3.04c-.41.41-1.09.41-1.5 0-.41-.41-.41-1.09 0-1.5L21 18l-2.25-2.25c-.41-.41-.41-1.09 0-1.5.41-.41 1.09-.41 1.5 0l3.04 3.04c.39.39.39 1.03 0 1.42z"/>
        </SvgIcon>
    }
}
