// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlightClassOutlined)]
pub fn flight_class_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="FlightClassOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M16,4h-2c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2V6C18,4.9,17.1,4,16,4z M16,11h-2V6h2V11z M9.5,16H18v2H9.49 c-0.88,0-1.66-0.58-1.92-1.43L5,8V4h2v4L9.5,16z M8,19h10v2H8V19z"/>
        </SvgIcon>
    }
}
