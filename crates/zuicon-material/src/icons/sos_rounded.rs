// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SosRounded)]
pub fn sos_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SosRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.5,7h-3c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h3c1.1,0,2-0.9,2-2V9C15.5,7.9,14.6,7,13.5,7z M13.5,15h-3V9h3V15z M3,9v2h2 c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2H2c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-2H3c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h3 c0.55,0,1,0.45,1,1S6.55,9,6,9H3z M19,9v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2h-3c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-2h-2 c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h3c0.55,0,1,0.45,1,1s-0.45,1-1,1H19z"/>
        </SvgIcon>
    }
}
