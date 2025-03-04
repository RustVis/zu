// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SafetyCheckRounded)]
pub fn safety_check_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SafetyCheckRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.3,2.26l-6,2.25C4.52,4.81,4,5.55,4,6.39v4.7c0,4.83,3.13,9.37,7.43,10.75c0.37,0.12,0.77,0.12,1.14,0 c4.3-1.38,7.43-5.91,7.43-10.75v-4.7c0-0.83-0.52-1.58-1.3-1.87l-6-2.25C12.25,2.09,11.75,2.09,11.3,2.26z M12,17 c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5S14.76,17,12,17z M14,14c-0.2,0.2-0.51,0.2-0.71,0l-1.65-1.65 c-0.09-0.09-0.15-0.22-0.15-0.35V9.5C11.5,9.22,11.72,9,12,9c0.28,0,0.5,0.22,0.5,0.5v2.29l1.5,1.5C14.2,13.49,14.2,13.8,14,14z"/>
        </SvgIcon>
    }
}
