// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiFindRounded)]
pub fn wifi_find_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiFindRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,14c0-3.36,2.64-6,6-6c2.2,0,4.08,1.13,5.13,2.86l0.36-0.37c0.86-0.86,0.76-2.27-0.2-3.01C19.44,5.3,15.87,4,12,4 C8.13,4,4.56,5.3,1.71,7.48c-0.96,0.74-1.06,2.15-0.2,3.01l9.08,9.09c0.78,0.78,2.05,0.78,2.83,0l0.45-0.45 C12.14,18.09,11,16.2,11,14z"/><path d="M20.44,16.03C20.79,15.44,21,14.75,21,14c0-2.24-1.76-4-4-4s-4,1.76-4,4c0,2.24,1.76,4,4,4c0.75,0,1.44-0.21,2.03-0.56 l1.85,1.85c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41L20.44,16.03z M17,16c-1.12,0-2-0.88-2-2c0-1.12,0.88-2,2-2 s2,0.88,2,2C19,15.12,18.12,16,17,16z"/>
        </SvgIcon>
    }
}
