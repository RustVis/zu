// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MinorCrashOutlined)]
pub fn minor_crash_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MinorCrashOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.92,9.01C18.72,8.42,18.16,8,17.5,8h-11C5.84,8,5.29,8.42,5.08,9.01L3,15v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8L18.92,9.01z M6.85,10h10.29l1.04,3H5.81L6.85,10z M19,20H5v-5h14V20z M6,17.5 C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S17.33,19,16.5,19S15,18.33,15,17.5z M9.41,5L8,6.41l-3-3L6.41,2L9.41,5z M16,6.41L14.59,5l3-3L19,3.41L16,6.41z M13,5h-2V0h2V5z"/>
        </SvgIcon>
    }
}
