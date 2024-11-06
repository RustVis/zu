// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditNotificationsOutlined)]
pub fn edit_notifications_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditNotificationsOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.58,6.25l1.77,1.77L14.37,13H12.6v-1.77L17.58,6.25z M20.85,5.81l-1.06-1.06c-0.2-0.2-0.51-0.2-0.71,0l-0.85,0.85 l1.77,1.77l0.85-0.85C21.05,6.32,21.05,6,20.85,5.81z M18,12.2V17h2v2H4v-2h2v-7c0-2.79,1.91-5.14,4.5-5.8V3.5 C10.5,2.67,11.17,2,12,2s1.5,0.67,1.5,1.5v0.7c0.82,0.21,1.57,0.59,2.21,1.09l-1.43,1.43C13.64,6.26,12.85,6,12,6 c-2.21,0-4,1.79-4,4v7h8v-2.8L18,12.2z M10,20h4c0,1.1-0.9,2-2,2S10,21.1,10,20z"/>
        </SvgIcon>
    }
}
