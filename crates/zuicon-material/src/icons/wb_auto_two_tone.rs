// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbAutoTwoTone)]
pub fn wb_auto_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WbAutoTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 6c-3.31 0-6 2.69-6 6s2.69 6 6 6c2.35 0 4.38-1.36 5.36-3.32l.01-.01c.4-.81.63-1.71.63-2.67 0-3.31-2.69-6-6-6zm2.3 10l-.7-2H6.4l-.7 2H3.8L7 7h2l3.2 9h-1.9zm-3.45-3.35h2.3L8 9z" opacity=".3"/><path d="M7 7l-3.2 9h1.9l.7-2h3.2l.7 2h1.9L9 7H7zm-.15 5.65L8 9l1.15 3.65h-2.3zm13.95.64L19.3 7h-1.6l-1.49 6.29L15 7h-.76l-.01.01C12.76 5.18 10.53 4 8 4c-4.42 0-8 3.58-8 8s3.58 8 8 8c2.96 0 5.55-1.61 6.93-4 .03-.06.05-.12.08-.18.05-.08.09-.17.14-.25l.1.43H17l1.5-6.1L20 16h1.75l2.05-9H22l-1.2 6.29zm-7.43 1.38C12.38 16.64 10.35 18 8 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6c0 .96-.23 1.86-.63 2.67z"/>
        </SvgIcon>
    }
}
