// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SportsEsportsSharp)]
pub fn sports_esports_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SportsEsportsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,5H4L2,19h4l3-3h6l3,3h4L20,5z M11,11H9v2H8v-2H6v-1h2V8h1v2h2V11z M15,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C16,9.55,15.55,10,15,10z M17,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C18,12.55,17.55,13,17,13z"/>
        </SvgIcon>
    }
}
