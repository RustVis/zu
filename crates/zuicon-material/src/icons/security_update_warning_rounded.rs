// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SecurityUpdateWarningRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SecurityUpdateWarningRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,13c0.55,0,1-0.45,1-1V8c0-0.55-0.45-1-1-1s-1,0.45-1,1v4C11,12.55,11.45,13,12,13z"/><path d="M17,1H7C5.9,1,5.01,1.9,5.01,3v18c0,1.1,0.89,2,1.99,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M17,18H7V6h10V18z"/>
        </SvgIcon>
    }
}