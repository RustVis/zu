// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TimerRounded)]
pub fn timer_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TimerRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M10,3h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,1,9,1.45,9,2C9,2.55,9.45,3,10,3z"/><path d="M19.03,7.39l0.75-0.75c0.38-0.38,0.39-1.01,0-1.4c0,0-0.01-0.01-0.01-0.01c-0.39-0.39-1.01-0.38-1.4,0l-0.75,0.75 C16.07,4.74,14.12,4,12,4c-4.8,0-8.88,3.96-9,8.76C2.87,17.84,6.94,22,12,22c4.98,0,9-4.03,9-9C21,10.88,20.26,8.93,19.03,7.39z M13,13c0,0.55-0.45,1-1,1s-1-0.45-1-1V9c0-0.55,0.45-1,1-1s1,0.45,1,1V13z"/>
        </SvgIcon>
    }
}
