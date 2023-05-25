// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MeetingRoomRounded)]
pub fn meeting_room_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MeetingRoomRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,19h-1V5c0-0.55-0.45-1-1-1h-4c0-0.55-0.45-1-1-1H6C5.45,3,5,3.45,5,4v15H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h9 c0.55,0,1-0.45,1-1V6h3v14c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1S20.55,19,20,19z M11,13L11,13c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C12,12.55,11.55,13,11,13z"/>
        </SvgIcon>
    }
}
