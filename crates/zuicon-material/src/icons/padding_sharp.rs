// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PaddingSharp)]
pub fn padding_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PaddingSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M3,3v18h18V3H3z M9,9H7V7h2V9z M13,9h-2V7h2V9z M17,9h-2V7h2V9z"/>
        </SvgIcon>
    }
}
