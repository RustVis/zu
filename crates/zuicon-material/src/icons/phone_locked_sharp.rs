// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhoneLockedSharp)]
pub fn phone_locked_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhoneLockedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,5V4c0-1.1-0.9-2-2-2s-2,0.9-2,2v1h-1v5h6V5H20z M19,5h-2V4c0-0.55,0.45-1,1-1s1,0.45,1,1V5z"/><path d="M21,15l-5-1l-2.9,2.9c-2.5-1.43-4.57-3.5-6-6L10,8L9,3L3,3c0,3.28,0.89,6.35,2.43,9c1.58,2.73,3.85,4.99,6.57,6.57 c2.65,1.53,5.72,2.43,9,2.43L21,15z"/>
        </SvgIcon>
    }
}
