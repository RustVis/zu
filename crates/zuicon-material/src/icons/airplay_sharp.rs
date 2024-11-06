// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirplaySharp)]
pub fn airplay_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirplaySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,22h12l-6-6L6,22z M23,3H1v16h6v-2H3V5h18v12h-4v2h6V3z"/>
        </SvgIcon>
    }
}
