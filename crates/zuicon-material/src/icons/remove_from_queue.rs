// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RemoveFromQueue)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="RemoveFromQueue"
            view_box={props.view_box.clone()}
            >
            <path d="M21,3H3C1.89,3,1,3.89,1,5v12c0,1.1,0.89,2,2,2h5v2h8v-2h5c1.1,0,1.99-0.9,1.99-2L23,5C23,3.89,22.1,3,21,3z M21,17H3V5 h18V17z M16,10v2H8v-2H16z"/>
        </SvgIcon>
    }
}
