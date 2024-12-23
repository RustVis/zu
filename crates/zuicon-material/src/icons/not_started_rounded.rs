// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NotStartedRounded)]
pub fn not_started_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NotStartedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M11,15c0,0.55-0.45,1-1,1s-1-0.45-1-1 V9c0-0.55,0.45-1,1-1s1,0.45,1,1V15z M16.02,12.78l-2.4,1.92C12.97,15.22,12,14.76,12,13.92v-3.84c0-0.84,0.97-1.3,1.62-0.78 l2.4,1.92C16.52,11.62,16.52,12.38,16.02,12.78z"/>
        </SvgIcon>
    }
}
