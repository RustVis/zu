// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SportsMma)]
pub fn sports_mma(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SportsMma"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,20c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-3H7V20z"/><path d="M18,7c-0.55,0-1,0.45-1,1V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v5.8c0,0.13,0.01,0.26,0.04,0.39l0.8,4 c0.09,0.47,0.5,0.8,0.98,0.8h10.36c0.45,0,0.89-0.36,0.98-0.8l0.8-4C18.99,11.06,19,10.93,19,10.8V8C19,7.45,18.55,7,18,7z M15,10 H7V7h8V10z"/>
        </SvgIcon>
    }
}
