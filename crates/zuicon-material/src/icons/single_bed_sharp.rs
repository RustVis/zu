// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SingleBedSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SingleBedSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M18,10V5H6v5H4v7h1.33L6,19h1l0.67-2h8.67L17,19h1l0.67-2H20v-7H18z M11,10H8V7h3V10z M16,10h-3V7h3V10z"/>
        </SvgIcon>
    }
}
