// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RemoveDoneTwoTone)]
pub fn remove_done_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RemoveDoneTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4.84,1.98L3.43,3.39l10.38,10.38l-1.41,1.41l-4.24-4.24l-1.41,1.41l5.66,5.66l2.83-2.83l6.6,6.6l1.41-1.41L4.84,1.98z M18.05,12.36L23,7.4L21.57,6l-4.94,4.94L18.05,12.36z M17.34,7.4l-1.41-1.41l-2.12,2.12l1.41,1.41L17.34,7.4z M1.08,12.35 l5.66,5.66l1.41-1.41l-5.66-5.66L1.08,12.35z"/>
        </SvgIcon>
    }
}
