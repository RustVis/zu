// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartScreenTwoTone)]
pub fn smart_screen_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartScreenTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,17h1V7H3V17z M20,7v10h1V7H20z" opacity=".3"/><path d="M14,11.25h-1.5v1.5H14V11.25z M16.5,11.25H15v1.5h1.5V11.25z M11.5,11.25H10v1.5h1.5V11.25z M9,11.25H7.5v1.5H9V11.25z M21,5H3C1.9,5,1,5.9,1,7v10c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V7C23,5.9,22.1,5,21,5z M4,17H3V7h1V17z M18,17H6V7h12V17z M21,17 h-1V7h1V17z"/>
        </SvgIcon>
    }
}
