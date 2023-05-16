// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Pinch)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Pinch"
            view_box={props.view_box.clone()}
            >
            <path d="M6,2.5V1h5v5H9.5V3.56L3.56,9.5H6V11H1V6h1.5v2.44L8.44,2.5H6z M22.98,16.82l-0.63,4.46C22.21,22.27,21.36,23,20.37,23 h-6.16c-0.53,0-1.29-0.21-1.66-0.59L8,17.62l0.83-0.84c0.24-0.24,0.58-0.35,0.92-0.28L13,17.24V6.5C13,5.67,13.67,5,14.5,5 S16,5.67,16,6.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04C22.66,15.14,23.1,15.97,22.98,16.82z"/>
        </SvgIcon>
    }
}