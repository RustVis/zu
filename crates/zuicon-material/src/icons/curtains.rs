// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Curtains)]
pub fn curtains(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Curtains"
            view_box={props.view_box.clone()}
            >
            <path d="M20,19V3H4v16H2v2h20v-2H20z M8.19,12c2.04-1.35,3.5-3.94,3.76-7h0.09c0.26,3.06,1.72,5.65,3.76,7 c-2.04,1.35-3.5,3.94-3.76,7h-0.09C11.69,15.94,10.23,13.35,8.19,12z"/>
        </SvgIcon>
    }
}
