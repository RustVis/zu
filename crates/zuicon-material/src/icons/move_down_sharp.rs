// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MoveDownSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MoveDownSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M3,11c0,2.45,1.76,4.47,4.08,4.91l-1.49-1.49L7,13l4,4.01L7,21l-1.41-1.41l1.58-1.58l0-0.06C3.7,17.54,1,14.58,1,11 c0-3.87,3.13-7,7-7h3v2H8C5.24,6,3,8.24,3,11z"/><path d="M22,11V4h-9v7H22z M20,9h-5V6h5V9z"/>
        </SvgIcon>
    }
}