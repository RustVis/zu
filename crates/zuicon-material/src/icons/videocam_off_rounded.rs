// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideocamOffRounded)]
pub fn videocam_off_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="VideocamOffRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M21 14.2V8.91c0-.89-1.08-1.34-1.71-.71L17 10.5V7c0-.55-.45-1-1-1h-5.61l8.91 8.91c.62.63 1.7.18 1.7-.71zM2.71 2.56c-.39.39-.39 1.02 0 1.41L4.73 6H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.21 0 .39-.08.55-.18l2.48 2.48c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.12 2.56c-.39-.39-1.02-.39-1.41 0z"/>
        </SvgIcon>
    }
}
