// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EighteenUpRatingTwoTone)]
pub fn eighteen_up_rating_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EighteenUpRatingTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M11.5,10c0-0.55,0.45-1,1-1H15c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1h-2.5 c-0.55,0-1-0.45-1-1V10z M7,9h3v6H8.5v-4.5H7V9z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M12.5,15H15c0.55,0,1-0.45,1-1v-4c0-0.55-0.45-1-1-1h-2.5c-0.55,0-1,0.45-1,1v4C11.5,14.55,11.95,15,12.5,15z M13,10h1.5 v1.5H13V10z M13,12.5h1.5V14H13V12.5z"/>
        </SvgIcon>
    }
}
