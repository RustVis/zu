// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewQuiltRounded)]
pub fn view_quilt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewQuiltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,6v4.5c0,0.55-0.45,1-1,1h-9.67c-0.55,0-1-0.45-1-1V6c0-0.55,0.45-1,1-1H20C20.55,5,21,5.45,21,6z M14.67,18v-4.5 c0-0.55-0.45-1-1-1h-3.33c-0.55,0-1,0.45-1,1V18c0,0.55,0.45,1,1,1h3.33C14.22,19,14.67,18.55,14.67,18z M15.67,13.5V18 c0,0.55,0.45,1,1,1H20c0.55,0,1-0.45,1-1v-4.5c0-0.55-0.45-1-1-1h-3.33C16.11,12.5,15.67,12.95,15.67,13.5z M8.33,18V6 c0-0.55-0.45-1-1-1H4C3.45,5,3,5.45,3,6v12c0,0.55,0.45,1,1,1h3.33C7.89,19,8.33,18.55,8.33,18z"/>
        </SvgIcon>
    }
}
