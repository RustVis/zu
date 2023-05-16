// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlightClassSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="FlightClassSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M18,4h-6v9h6V4z M9.5,16H18v2H8L5,8V4h2v4L9.5,16z M8,19h10v2H8V19z"/>
        </SvgIcon>
    }
}
