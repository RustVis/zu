// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkEmailReadSharp)]
pub fn mark_email_read_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkEmailReadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,19c0-3.87,3.13-7,7-7c1.08,0,2.09,0.25,3,0.68V4H2v16h10.08C12.03,19.67,12,19.34,12,19z M4,6l8,5l8-5v2l-8,5L4,8V6z M17.34,22l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24L23,16.34L17.34,22z"/>
        </SvgIcon>
    }
}
