// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiCallingTwoTone)]
pub fn wifi_calling_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiCallingTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.2,18.21c1.2,0.41,2.48,0.67,3.8,0.75v-1.49c-0.88-0.07-1.75-0.22-2.6-0.45L15.2,18.21z" opacity=".3"/><path d="M6.54,5h-1.5c0.09,1.32,0.35,2.59,0.75,3.8l1.2-1.2C6.75,6.76,6.6,5.89,6.54,5z" opacity=".3"/><path d="M20,15.51c-1.24,0-2.45-0.2-3.57-0.57c-0.1-0.04-0.21-0.05-0.31-0.05c-0.26,0-0.51,0.1-0.71,0.29l-2.2,2.2 c-2.83-1.45-5.15-3.76-6.59-6.59l2.2-2.2C9.1,8.31,9.18,7.92,9.07,7.57C8.7,6.45,8.5,5.25,8.5,4c0-0.55-0.45-1-1-1H4 C3.45,3,3,3.45,3,4c0,9.39,7.61,17,17,17c0.55,0,1-0.45,1-1v-3.49C21,15.96,20.55,15.51,20,15.51z M5.03,5h1.5 C6.6,5.89,6.75,6.76,6.99,7.59l-1.2,1.2C5.38,7.59,5.12,6.32,5.03,5z M19,18.97c-1.32-0.09-2.59-0.35-3.8-0.75l1.19-1.19 c0.85,0.24,1.72,0.39,2.6,0.45V18.97z"/><path d="M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z"/>
        </SvgIcon>
    }
}
