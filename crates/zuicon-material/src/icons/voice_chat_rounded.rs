// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VoiceChatRounded)]
pub fn voice_chat_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VoiceChatRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-3.62 10.7L14 10.8V13c0 .55-.45 1-1 1H7c-.55 0-1-.45-1-1V7c0-.55.45-1 1-1h6c.55 0 1 .45 1 1v2.2l2.38-1.9c.65-.52 1.62-.06 1.62.78v3.84c0 .84-.97 1.3-1.62.78z"/>
        </SvgIcon>
    }
}
