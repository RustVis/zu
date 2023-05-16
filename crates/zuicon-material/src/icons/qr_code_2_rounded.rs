// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(QrCode2Rounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="QrCode2Rounded"
            view_box={props.view_box.clone()}
            >
            <path d="M15,21h-2v-2h2V21z M13,14h-2v5h2V14z M21,12h-2v4h2V12z M19,10h-2v2h2V10z M7,12H5v2h2V12z M5,10H3v2h2V10z M12,5h2V3h-2V5 z M4.5,4.5v3h3v-3H4.5z M8,9H4C3.45,9,3,8.55,3,8V4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C9,8.55,8.55,9,8,9z M4.5,16.5v3h3v-3 H4.5z M8,21H4c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C9,20.55,8.55,21,8,21z M16.5,4.5v3h3v-3H16.5z M20,9 h-4c-0.55,0-1-0.45-1-1V4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C21,8.55,20.55,9,20,9z M19,19v-3l-4,0v2h2v3h4v-2H19z M17,12 l-4,0v2h4V12z M13,10H7v2h2v2h2v-2h2V10z M14,9V7h-2V5h-2v4L14,9z M6.75,5.25h-1.5v1.5h1.5V5.25z M6.75,17.25h-1.5v1.5h1.5V17.25z M18.75,5.25h-1.5v1.5h1.5V5.25z"/>
        </SvgIcon>
    }
}