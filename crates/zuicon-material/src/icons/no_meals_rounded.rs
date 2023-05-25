// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoMealsRounded)]
pub fn no_meals_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NoMealsRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M21,18.17l-2-2V14h-1c-1.1,0-2-0.9-2-2V6c0-1.49,1.6-3.32,3.76-3.85C20.39,2,21,2.48,21,3.13V18.17z M21.19,22.61 c-0.39,0.39-1.02,0.39-1.41,0l-9.76-9.76C9.69,12.94,9.36,13,9,13v8c0,0.55-0.45,1-1,1s-1-0.45-1-1v-8c-2.21,0-4-1.79-4-4V5.83 L1.39,4.22C1,3.83,1,3.2,1.39,2.81c0.39-0.39,1.02-0.39,1.41,0l18.38,18.38C21.58,21.58,21.58,22.22,21.19,22.61z M6.17,9L5,7.83V9 H6.17z M13,9V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v5.17l1.85,1.85C12.94,9.69,13,9.36,13,9z M9,3c0-0.55-0.45-1-1-1S7,2.45,7,3v1.17l2,2 V3z"/>
        </SvgIcon>
    }
}
