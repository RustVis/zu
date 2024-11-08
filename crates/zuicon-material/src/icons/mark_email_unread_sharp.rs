// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkEmailUnreadSharp)]
pub fn mark_email_unread_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkEmailUnreadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,8.98V20H2V4h12.1C14.04,4.32,14,4.66,14,5c0,1.48,0.65,2.79,1.67,3.71L12,11L4,6v2l8,5l5.3-3.32 C17.84,9.88,18.4,10,19,10C20.13,10,21.16,9.61,22,8.98z M16,5c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,3.34,16,5z"/>
        </SvgIcon>
    }
}
