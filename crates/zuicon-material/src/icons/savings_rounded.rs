// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SavingsRounded)]
pub fn savings_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SavingsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.83,7.5l-2.27-2.27c0.07-0.42,0.18-0.81,0.32-1.15c0.11-0.26,0.15-0.56,0.09-0.87C17.84,2.49,17.14,1.99,16.4,2 c-1.59,0.03-3,0.81-3.9,2l-5,0C4.46,4,2,6.46,2,9.5c0,2.25,1.37,7.48,2.08,10.04C4.32,20.4,5.11,21,6.01,21L8,21c1.1,0,2-0.9,2-2v0 h2v0c0,1.1,0.9,2,2,2l2.01,0c0.88,0,1.66-0.58,1.92-1.43l1.25-4.16l2.14-0.72c0.41-0.14,0.68-0.52,0.68-0.95V8.5c0-0.55-0.45-1-1-1 H19.83z M12,9H9C8.45,9,8,8.55,8,8v0c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v0C13,8.55,12.55,9,12,9z M16,11c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C17,10.55,16.55,11,16,11z"/>
        </SvgIcon>
    }
}
