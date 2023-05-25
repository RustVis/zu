// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoDrinks)]
pub fn no_drinks(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NoDrinks"
            view_box={props.view_box.clone()}
            >
            <path d="M5.83,3H21v2l-6.2,6.97L9.83,7h6.74l1.78-2H7.83L5.83,3z M19.78,22.61L18,20.83V21H6v-2h5v-5l-1.37-1.54L1.39,4.22 l1.41-1.41L3,3l18.19,18.19L19.78,22.61z M16.17,19L13,15.83V19H16.17z"/>
        </SvgIcon>
    }
}
