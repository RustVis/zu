// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RsvpTwoTone)]
pub fn rsvp_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="RsvpTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,9h1.5l-1.75,6h-1.5L12.5,9H14l1,3.43L16,9z M5.1,12.9L6,15H4.5l-0.85-2H2.5v2H1V9h3.5C5.35,9,6,9.65,6,10.5v1 C6,12.1,5.6,12.65,5.1,12.9z M4.5,10.5h-2v1h2V10.5z M21.5,13h-2v2H18V9h3.5c0.83,0,1.5,0.67,1.5,1.5v1C23,12.33,22.33,13,21.5,13 z M21.5,10.5h-2v1h2V10.5z M11.5,9v1.5h-3v0.75h2c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1H7v-1.5h3v-0.75H7.75 C7.34,12.75,7,12.41,7,12v-2c0-0.55,0.45-1,1-1H11.5z"/>
        </SvgIcon>
    }
}
