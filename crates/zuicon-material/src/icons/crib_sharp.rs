// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CribSharp)]
pub fn crib_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CribSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M20,9h-8V4H8C5.79,4,4,5.79,4,8v8h4v2.93c-0.61-0.35-1.16-0.78-1.65-1.27l-1.42,1.42C6.74,20.88,9.24,22,12,22 c2.76,0,5.26-1.12,7.07-2.93l-1.42-1.42c-0.49,0.49-1.05,0.92-1.65,1.27V16h4V9z M14,19.75C13.36,19.91,12.69,20,12,20 c-0.69,0-1.36-0.09-2-0.25V16h4V19.75z"/>
        </SvgIcon>
    }
}
