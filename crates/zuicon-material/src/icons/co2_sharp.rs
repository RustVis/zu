// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Co2Sharp)]
pub fn co2_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Co2Sharp"
            view_box={props.view_box.clone()}
            >
            <path d="M15,9h-5v6h5V9z M13.5,13.5h-2v-3h2V13.5z M8,13v2H3V9h5v2H6.5v-0.5h-2v3h2V13H8z M18.5,15.5v1h3V18H17v-3.5h3v-1h-3V12h4.5 v3.5H18.5z"/>
        </SvgIcon>
    }
}
