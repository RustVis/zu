// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HomeMaxRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="HomeMaxRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M19,5H5C2.79,5,1,6.79,1,9v5c0,2.21,1.79,4,4,4h2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1h2c2.21,0,4-1.79,4-4V9 C23,6.79,21.21,5,19,5z M21,14c0,1.1-0.9,2-2,2H5c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h14c1.1,0,2,0.9,2,2V14z"/>
        </SvgIcon>
    }
}
