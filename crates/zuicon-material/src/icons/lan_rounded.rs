// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LanRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="LanRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M15,22h4c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-1v-2c0-1.1-0.9-2-2-2h-3V9h1c1.1,0,2-0.9,2-2V4c0-1.1-0.9-2-2-2h-4 C8.9,2,8,2.9,8,4v3c0,1.1,0.9,2,2,2h1v2H8c-1.1,0-2,0.9-2,2v2H5c-1.1,0-2,0.9-2,2v3c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2v-3 c0-1.1-0.9-2-2-2H8v-2h8v2h-1c-1.1,0-2,0.9-2,2v3C13,21.1,13.9,22,15,22z"/>
        </SvgIcon>
    }
}