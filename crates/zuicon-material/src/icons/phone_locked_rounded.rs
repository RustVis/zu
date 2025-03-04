// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhoneLockedRounded)]
pub fn phone_locked_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhoneLockedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,5V4.11c0-1-0.68-1.92-1.66-2.08C17.08,1.82,16,2.79,16,4v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1V6C21,5.45,20.55,5,20,5z M19,5h-2V4c0-0.55,0.45-1,1-1s1,0.45,1,1V5z"/><path d="M15.63,14.4l-2.52,2.5c-2.5-1.43-4.57-3.5-6-6l2.5-2.52c0.23-0.24,0.33-0.57,0.27-0.9L9.13,3.8C9.04,3.34,8.63,3,8.15,3 L4,3C3.44,3,2.97,3.47,3,4.03C3.17,6.92,4.05,9.63,5.43,12c1.58,2.73,3.85,4.99,6.57,6.57c2.37,1.37,5.08,2.26,7.97,2.43 c0.56,0.03,1.03-0.44,1.03-1l0-4.15c0-0.48-0.34-0.89-0.8-0.98l-3.67-0.73C16.2,14.07,15.86,14.17,15.63,14.4z"/>
        </SvgIcon>
    }
}
