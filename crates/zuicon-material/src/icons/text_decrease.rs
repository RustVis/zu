// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextDecrease)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TextDecrease"
            view_box={props.view_box.clone()}
            >
            <path d="M0.99,19h2.42l1.27-3.58h5.65L11.59,19h2.42L8.75,5h-2.5L0.99,19z M5.41,13.39L7.44,7.6h0.12l2.03,5.79H5.41z M23,11v2h-8 v-2H23z"/>
        </SvgIcon>
    }
}