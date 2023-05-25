// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenshotMonitorTwoTone)]
pub fn screenshot_monitor_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ScreenshotMonitorTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M4,17h16V5H4V17z M15,14.5h2.5V12H19v4h-4V14.5z M5,6h4v1.5H6.5V10H5V6z" opacity=".3"/><path d="M20,3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2V5C22,3.89,21.1,3,20,3z M20,17H4V5h16V17z"/>
        </SvgIcon>
    }
}
