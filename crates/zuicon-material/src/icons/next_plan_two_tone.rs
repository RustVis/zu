// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NextPlanTwoTone)]
pub fn next_plan_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NextPlanTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S16.42,4,12,4z M13,14l2.26-2.26 C14.35,10.68,13.01,10,11.5,10c-2.37,0-4.35,1.66-4.86,3.88l-0.96-0.32C6.32,10.95,8.68,9,11.5,9c1.78,0,3.37,0.79,4.47,2.03L18,9 v5H13z" opacity=".3"/><path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8 s3.58-8,8-8s8,3.58,8,8S16.42,20,12,20z"/><path d="M15.97,11.03C14.87,9.79,13.28,9,11.5,9c-2.82,0-5.18,1.95-5.82,4.56l0.96,0.32C7.15,11.66,9.13,10,11.5,10 c1.51,0,2.85,0.68,3.76,1.74L13,14h5V9L15.97,11.03z"/>
        </SvgIcon>
    }
}
