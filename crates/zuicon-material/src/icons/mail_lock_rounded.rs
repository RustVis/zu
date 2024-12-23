// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MailLockRounded)]
pub fn mail_lock_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MailLockRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,9.97V6c0-1.1-0.9-2-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h12v-5.03c0-2.76,2.24-5,5-5H22z M19.6,8.25 l-6.54,4.09c-0.65,0.41-1.47,0.41-2.12,0L4.4,8.25C4.15,8.09,4,7.82,4,7.53v0c0-0.67,0.73-1.07,1.3-0.72L12,11l6.7-4.19 C19.27,6.46,20,6.86,20,7.53v0C20,7.82,19.85,8.09,19.6,8.25z"/><path d="M23,15v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C24,15.45,23.55,15,23,15z M22,15h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V15z"/>
        </SvgIcon>
    }
}
