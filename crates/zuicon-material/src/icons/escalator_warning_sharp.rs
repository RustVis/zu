// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EscalatorWarningSharp)]
pub fn escalator_warning_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EscalatorWarningSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.5,2c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S5.4,2,6.5,2z M15.5,9.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 S17.83,8,17,8S15.5,8.67,15.5,9.5z M14.21,12.86l-0.92,1.32L9.72,8C9.37,7.38,8.71,7,7.99,7H3v8h1.5v7h5V11.61L12.03,16h2.2 L15,14.9V22h4v-5h1v-5h-4.15C15.19,12,14.58,12.32,14.21,12.86z"/>
        </SvgIcon>
    }
}
