// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DisplaySettingsTwoTone)]
pub fn display_settings_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DisplaySettingsTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,17h16V5H4V17z M18,13.75h-8v-1.5h8V13.75z M15,7h1.5v1.25H18v1.5h-1.5V11H15V7z M6,8.25h8v1.5H6V8.25z M6,12.25h1.5V11H9v4H7.5v-1.25H6V12.25z" opacity=".3"/><path d="M20,3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2V5C22,3.89,21.1,3,20,3z M20,17H4V5h16V17z"/>
        </SvgIcon>
    }
}
