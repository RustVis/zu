// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlatwareSharp)]
pub fn flatware_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlatwareSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,7.08c0,1.77-0.84,3.25-2,3.82V21h-2V10.9c-1.16-0.57-2-2.05-2-3.82C10.01,4.83,11.35,3,13,3C14.66,3,16,4.83,16,7.08z M17,3v18h2v-8h2V7C21,5.24,19.76,3,17,3z M8.28,3H7.56v4H6.72V3L6,3L5.28,3v4H4.44V3L3.72,3H3v8c0,0,0.9,0,2,0v10h2V11 c1.1,0,2,0,2,0V3L8.28,3z"/>
        </SvgIcon>
    }
}
