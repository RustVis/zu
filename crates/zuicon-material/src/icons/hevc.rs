// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Hevc)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Hevc"
            view_box={props.view_box.clone()}
            >
            <path d="M21,11v-1c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-1h-1.5v0.5h-1v-3h1V11H21z"/>
        </SvgIcon>
    }
}