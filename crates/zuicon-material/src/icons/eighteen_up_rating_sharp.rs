// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EighteenUpRatingSharp)]
pub fn eighteen_up_rating_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EighteenUpRatingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M10,15H8.5v-4.5H7V9h3V15z M16,14c0,0.55-0.45,1-1,1h-2.5c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1H15 c0.55,0,1,0.45,1,1V14z"/>
        </SvgIcon>
    }
}
