// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TourRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TourRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M20.45,5.37C20.71,4.71,20.23,4,19.52,4H13h-1H7V3c0-0.55-0.45-1-1-1h0C5.45,2,5,2.45,5,3v1v10v8h2v-8h4h1h7.52 c0.71,0,1.19-0.71,0.93-1.37L19,9L20.45,5.37z M15,9c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S15,7.9,15,9z"/>
        </SvgIcon>
    }
}