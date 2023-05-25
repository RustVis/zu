// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SyncAltRounded)]
pub fn sync_alt_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SyncAltRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M21.65,7.65l-2.79-2.79C18.54,4.54,18,4.76,18,5.21V7H4C3.45,7,3,7.45,3,8s0.45,1,1,1h14v1.79c0,0.45,0.54,0.67,0.85,0.35 l2.79-2.79C21.84,8.16,21.84,7.84,21.65,7.65z"/><path d="M20,15H6v-1.79c0-0.45-0.54-0.67-0.85-0.35l-2.79,2.79c-0.2,0.19-0.2,0.51-0.01,0.7l2.79,2.79C5.46,19.46,6,19.24,6,18.79 V17h14c0.55,0,1-0.45,1-1S20.55,15,20,15z"/>
        </SvgIcon>
    }
}
