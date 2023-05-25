// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SnippetFolderRounded)]
pub fn snippet_folder_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SnippetFolderRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M15.88,10.5l1.62,1.62v3.38l-3,0v-5H15.88z M22,8v10c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2L2.01,6C2.01,4.9,2.9,4,4,4h5.17 c0.53,0,1.04,0.21,1.41,0.59L12,6h8C21.1,6,22,6.9,22,8z M19,11.91c0-0.27-0.11-0.52-0.29-0.71l-1.91-1.91 C16.61,9.11,16.35,9,16.09,9H14c-0.55,0-1,0.45-1,1v6c0,0.55,0.45,1,1,1l4,0c0.55,0,1-0.45,1-1V11.91z"/>
        </SvgIcon>
    }
}
