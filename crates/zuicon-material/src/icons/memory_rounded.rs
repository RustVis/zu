// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MemoryRounded)]
pub fn memory_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MemoryRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14 9h-4c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 4h-2v-2h2v2zm8-3c0-.55-.45-1-1-1h-1V7c0-1.1-.9-2-2-2h-2V4c0-.55-.45-1-1-1s-1 .45-1 1v1h-2V4c0-.55-.45-1-1-1s-1 .45-1 1v1H7c-1.1 0-2 .9-2 2v2H4c-.55 0-1 .45-1 1s.45 1 1 1h1v2H4c-.55 0-1 .45-1 1s.45 1 1 1h1v2c0 1.1.9 2 2 2h2v1c0 .55.45 1 1 1s1-.45 1-1v-1h2v1c0 .55.45 1 1 1s1-.45 1-1v-1h2c1.1 0 2-.9 2-2v-2h1c.55 0 1-.45 1-1s-.45-1-1-1h-1v-2h1c.55 0 1-.45 1-1zm-5 7H8c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1h8c.55 0 1 .45 1 1v8c0 .55-.45 1-1 1z"/>
        </SvgIcon>
    }
}
