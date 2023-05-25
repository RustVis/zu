// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TryRounded)]
pub fn try_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TryRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M20,2H4C2.9,2,2,2.9,2,4v15.59c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M13.57,11.57 l-1.12,2.44c-0.18,0.39-0.73,0.39-0.91,0l-1.12-2.44l-2.44-1.12c-0.39-0.18-0.39-0.73,0-0.91l2.44-1.12l1.12-2.44 c0.18-0.39,0.73-0.39,0.91,0l1.12,2.44l2.44,1.12c0.39,0.18,0.39,0.73,0,0.91L13.57,11.57z"/>
        </SvgIcon>
    }
}
