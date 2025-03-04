// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignLanguage)]
pub fn sign_language(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignLanguage"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.49,13l-0.93-1.86c-0.37-0.74-0.07-1.64,0.67-2.01L12.49,9l5.73,5.46c0.5,0.47,0.78,1.13,0.78,1.81v5.23 c0,1.38-1.12,2.5-2.5,2.5h-11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1H10v-1H4c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h6v-1H3 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h7v-1H4.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1H12.49z M11.78,7.12 c-0.84,0.4-1.17,0.62-1.63,1.19l-2.7-2.85c-0.38-0.4-0.36-1.03,0.04-1.41c0.4-0.38,1.03-0.36,1.41,0.04L11.78,7.12z M9.64,9.21 C9.41,9.76,9.35,10.45,9.44,11H8.58L6.31,8.61C5.93,8.21,5.94,7.58,6.35,7.2c0.4-0.38,1.03-0.36,1.41,0.04L9.64,9.21z M20.33,13.91 l0.88-0.83c0.5-0.47,0.79-1.13,0.79-1.82V3.35l-0.27-0.1c-0.78-0.28-1.64,0.12-1.92,0.9L19.1,6.11l-5.5-5.8 c-0.38-0.4-1.01-0.42-1.41-0.04c-0.4,0.38-0.42,1.01-0.04,1.41l3.79,3.99l-0.73,0.69l-4.82-5.08c-0.38-0.4-1.01-0.42-1.41-0.04 c-0.4,0.38-0.42,1.01-0.04,1.41l3.78,3.98L15.38,9l3.61,3.43l0.61,0.58C19.89,13.28,20.13,13.58,20.33,13.91z"/>
        </SvgIcon>
    }
}
