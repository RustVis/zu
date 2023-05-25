// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ControlCameraRounded)]
pub fn control_camera_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ControlCameraRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M4.65 9.35L2.7 11.3c-.39.39-.39 1.02 0 1.41l1.95 1.95c.49.49 1.28.49 1.77 0 .48-.49.48-1.27 0-1.76l-.88-.9.88-.89c.48-.49.48-1.27 0-1.76s-1.28-.49-1.77 0zm12.93 0c-.48.49-.48 1.27 0 1.76l.88.89-.88.89c-.48.49-.48 1.27 0 1.76.49.49 1.28.49 1.77 0l1.95-1.95c.39-.39.39-1.02 0-1.41l-1.95-1.95c-.49-.48-1.29-.48-1.77.01zM12 18.46l-.89-.88c-.49-.48-1.27-.48-1.76 0-.49.49-.49 1.28 0 1.77l1.95 1.95c.39.39 1.02.39 1.41 0l1.95-1.95c.49-.49.49-1.28 0-1.77-.49-.48-1.27-.48-1.76 0l-.9.88zM9.35 6.42c.49.48 1.27.48 1.76 0l.89-.88.89.88c.49.48 1.27.48 1.76 0 .49-.49.49-1.28 0-1.77L12.7 2.7c-.39-.39-1.02-.39-1.41 0L9.35 4.65c-.49.49-.49 1.29 0 1.77z"/>
        </SvgIcon>
    }
}
