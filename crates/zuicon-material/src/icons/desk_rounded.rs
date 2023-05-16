// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeskRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="DeskRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M2,7v10c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V8h10v9c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h4v1c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1H3C2.45,6,2,6.45,2,7z M20,8v2h-4V8H20z M16,14v-2h4v2H16z"/>
        </SvgIcon>
    }
}
