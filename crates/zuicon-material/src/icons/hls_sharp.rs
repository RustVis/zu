// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HlsSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="HlsSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M6.5,9H8v6H6.5v-2.5h-2V15H3V9h1.5v2h2V9z M15.5,15h5v-3.5H17v-1h2V11h1.5V9h-5v3.5H19v1h-2V13h-1.5V15z M14,15v-1.5h-2.5 V9H10v6H14z"/>
        </SvgIcon>
    }
}