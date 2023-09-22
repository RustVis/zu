// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RampRightRounded)]
pub fn ramp_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RampRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,21c0.55,0,1-0.45,1-1V6.83l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59 c-0.39-0.39-1.02-0.39-1.41,0L8.71,6.29c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L11,6.83v0V9 c0,3.62-2.89,6.22-4.97,7.62c-0.52,0.35-0.59,1.09-0.14,1.53c0.33,0.33,0.87,0.4,1.26,0.13c1.59-1.06,2.89-2.28,3.85-3.59l0,5.3 C11,20.55,11.45,21,12,21z"/>
        </SvgIcon>
    }
}
