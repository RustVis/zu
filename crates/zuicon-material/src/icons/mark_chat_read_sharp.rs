// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkChatReadSharp)]
pub fn mark_chat_read_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkChatReadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.03,17.5C12.01,17.67,12,17.83,12,18v0H6l-4,4V2h20v8.68C21.09,10.25,20.08,10,19,10c-3.87,0-7,3.13-7,7 C12,17.17,12.01,17.33,12.03,17.5z M23,14.34l-1.41-1.41l-4.24,4.24l-2.12-2.12l-1.41,1.41L17.34,20L23,14.34z"/>
        </SvgIcon>
    }
}
