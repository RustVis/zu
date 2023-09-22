// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SeventeenMpTwoTone)]
pub fn seventeen_mp_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SeventeenMpTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M18,13.5V16c0,0.55-0.45,1-1,1h-2v1.5h-1.5v-6H17C17.55,12.5,18,12.95,18,13.5z M12,5.5 h3.5c0.67,0,1.15,0.65,0.96,1.29L15,11.5h-1.75L14.62,7H12V5.5z M7,5.5h3v6H8.5V7H7V5.5z M6,13.5c0-0.55,0.45-1,1-1h4.5 c0.55,0,1,0.45,1,1v5H11V14h-1v3H8.5v-3h-1v4.5H6V13.5z" opacity=".3"/><path d="M7.5,14h1v3H10v-3h1v4.5h1.5v-5c0-0.55-0.45-1-1-1H7c-0.55,0-1,0.45-1,1v5h1.5V14z"/><path d="M13.5,18.5H15V17h2c0.55,0,1-0.45,1-1v-2.5c0-0.55-0.45-1-1-1h-3.5V18.5z M15,14h1.5v1.5H15V14z"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M13.25,11.5H15l1.46-4.71c0.19-0.64-0.29-1.29-0.96-1.29H12V7h2.62L13.25,11.5z"/>
        </SvgIcon>
    }
}
