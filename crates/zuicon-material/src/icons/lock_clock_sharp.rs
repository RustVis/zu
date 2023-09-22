// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LockClockSharp)]
pub fn lock_clock_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LockClockSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,11c0.7,0,1.37,0.1,2,0.29V8h-3V6.21c0-2.61-1.91-4.94-4.51-5.19C9.51,0.74,7,3.08,7,6v2H4v14h8.26 C11.47,20.87,11,19.49,11,18C11,14.13,14.13,11,18,11z M9,6c0-1.66,1.34-3,3-3s3,1.34,3,3v2H9V6z M18,13c-2.76,0-5,2.24-5,5 s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M19.65,20.35l-2.15-2.15V15h1v2.79l1.85,1.85L19.65,20.35z"/>
        </SvgIcon>
    }
}
