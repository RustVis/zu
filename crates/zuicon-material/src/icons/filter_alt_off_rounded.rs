// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilterAltOffRounded)]
pub fn filter_alt_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilterAltOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.79,5.61C20.3,4.95,19.83,4,19,4H6.83l7.97,7.97L19.79,5.61z"/><path d="M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L10,13v5c0,1.1,0.9,2,2,2s2-0.9,2-2 v-1.17l5.07,5.07c0.39,0.39,1.02,0.39,1.41,0S20.88,20.88,20.49,20.49z"/>
        </SvgIcon>
    }
}
