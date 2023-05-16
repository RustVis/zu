// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoAlbumSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PhotoAlbumSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M20,2H4v20h16V2z M11,4h5v7l-2.5-1.5L11,11V4z M7,18l2.38-3.17L11,17l2.62-3.5L17,18H7z"/>
        </SvgIcon>
    }
}
