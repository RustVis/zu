// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BedroomBabySharp)]
pub fn bedroom_baby_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BedroomBabySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,2H2v20h20V2z M12,18c-2.64,0-5.13-1.03-7-2.9l1.06-1.06c0.34,0.34,0.71,0.65,1.1,0.92L8,13.5V9.51L6.45,10.5l-0.95-1 L7,7.76L6,7h3.65l1.73,3H17v1h-1v2.5l0.84,1.46c0.39-0.28,0.76-0.58,1.1-0.92L19,15.1C17.13,16.97,14.64,18,12,18z"/><path d="M14.69,14.24c-1.74,0.65-3.66,0.65-5.4,0l-0.81,1.41l-0.03,0.06c1.1,0.52,2.28,0.79,3.53,0.79s2.45-0.28,3.55-0.79 l-0.03-0.06L14.69,14.24z"/>
        </SvgIcon>
    }
}
