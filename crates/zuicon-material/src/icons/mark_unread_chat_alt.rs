// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkUnreadChatAlt)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MarkUnreadChatAlt"
            view_box={props.view_box.clone()}
            >
            <path d="M6,8V6h9.03c-1.21-1.6-1.08-3.21-0.92-4H4.01c-1.1,0-2,0.89-2,2L2,22l4-4h14c1.1,0,2-0.9,2-2V6.97 C21.16,7.61,20.13,8,19,8H6z M14,14H6v-2h8V14z M18,11H6V9h12V11z"/>
        </SvgIcon>
    }
}
