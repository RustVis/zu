// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SsidChartOutlined)]
pub fn ssid_chart_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SsidChartOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M21,5.47L12,12L7.62,7.62L3,11V8.52L7.83,5l4.38,4.38L21,3L21,5.47z M21,15h-4.7l-4.17,3.34L6,12.41l-3,2.13L3,17l2.8-2 l6.2,6l5-4h4V15z"/>
        </SvgIcon>
    }
}
