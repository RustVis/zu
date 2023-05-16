// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HtmlSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="HtmlSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M3.5,9H5v6H3.5v-2.5h-2V15H0V9h1.5v2h2V9z M18.5,9H12v6h1.5v-4.5h1V14H16v-3.51h1V15h1.5V9z M11,9H6v1.5h1.75V15h1.5v-4.5 H11V9z M24,15v-1.5h-2.5V9H20v6H24z"/>
        </SvgIcon>
    }
}
