// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowserUpdated)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="BrowserUpdated"
            view_box={props.view_box.clone()}
            >
            <path d="M22,13v3c0,1.1-0.9,2-2,2h-3l1,1v2H6v-2l1-1H4c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2l8,0v2L4,5v11h16v-3H22z M15,15l-5-5h4V3 h2v7h4L15,15z"/>
        </SvgIcon>
    }
}
