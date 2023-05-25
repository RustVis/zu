// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowCircleLeft)]
pub fn arrow_circle_left(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ArrowCircleLeft"
            view_box={props.view_box.clone()}
            >
            <path d="M2,12c0,5.52,4.48,10,10,10s10-4.48,10-10c0-5.52-4.48-10-10-10C6.48,2,2,6.48,2,12z M12,11l4,0v2l-4,0l0,3l-4-4l4-4L12,11 z"/>
        </SvgIcon>
    }
}
