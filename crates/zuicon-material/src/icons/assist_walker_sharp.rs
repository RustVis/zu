// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AssistWalkerSharp)]
pub fn assist_walker_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AssistWalkerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.77,17.72L19,10h-3c-1.5-0.02-2.86-0.54-3.76-1.44l-2-1.98C10.08,6.42,9.62,6,8.83,6C8.32,6,7.81,6.2,7.42,6.59 l-4.2,4.17l2.08,4.07l-3.15,4.05l1.57,1.24l3.68-4.73l-0.17-1.36L8,14.75V20h2v-6.12l-2.12-2.12l2.36-2.36 c0.94,0.94,1.72,1.82,3.59,2.32L13,20h1.5l0.41-3.5h3.18l0.14,1.22c-0.44,0.26-0.73,0.74-0.73,1.28c0,0.83,0.67,1.5,1.5,1.5 s1.5-0.67,1.5-1.5C20.5,18.46,20.21,17.98,19.77,17.72z M15.09,15l0.41-3.5h2l0.41,3.5H15.09z"/>
        </SvgIcon>
    }
}
