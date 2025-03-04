// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GirlRounded)]
pub fn girl_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GirlRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,7.5c0.97,0,1.75-0.78,1.75-1.75S12.97,4,12,4s-1.75,0.78-1.75,1.75S11.03,7.5,12,7.5z M14,16v3c0,0.55-0.45,1-1,1h-2 c-0.55,0-1-0.45-1-1v-3H9.44c-0.7,0-1.18-0.7-0.94-1.35l1.88-5.03C10.63,8.95,11.28,8.5,12,8.5s1.37,0.45,1.62,1.12l1.88,5.03 C15.74,15.3,15.26,16,14.56,16H14z"/>
        </SvgIcon>
    }
}
