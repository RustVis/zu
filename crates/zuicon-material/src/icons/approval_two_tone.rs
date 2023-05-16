// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ApprovalTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ApprovalTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M12,4c-1.66,0-3,1.34-3,3l3,4l3-4C15,5.34,13.66,4,12,4z" opacity=".3"/><path d="M12,2C9.24,2,7,4.24,7,7l5,7l5-7C17,4.24,14.76,2,12,2z M12,11L9,7c0-1.66,1.34-3,3-3s3,1.34,3,3L12,11z"/><path d="M18,14h-6H6c-1.1,0-2,0.9-2,2v6h16v-6C20,14.9,19.1,14,18,14z M18,18H6v-2h12V18z"/>
        </SvgIcon>
    }
}
