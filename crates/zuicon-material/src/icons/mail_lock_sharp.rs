// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MailLockSharp)]
pub fn mail_lock_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MailLockSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,9.97V4H2.01L2,20h14v-5.03c0-2.76,2.24-5,5-5H22z M20,8l-8,5L4,8V6l8,5l8-5V8z"/><path d="M23,15v-0.89c0-1-0.68-1.92-1.66-2.08C20.08,11.82,19,12.79,19,14v1h-1v5h6v-5H23z M22,15h-2v-1c0-0.55,0.45-1,1-1 s1,0.45,1,1V15z"/>
        </SvgIcon>
    }
}
