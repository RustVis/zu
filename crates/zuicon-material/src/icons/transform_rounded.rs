// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TransformRounded)]
pub fn transform_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TransformRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 16H9c-.55 0-1-.45-1-1V4h.79c.45 0 .67-.54.35-.85l-1.79-1.8c-.2-.2-.51-.2-.71 0l-1.79 1.8c-.31.31-.09.85.36.85H6v2H3c-.55 0-1 .45-1 1s.45 1 1 1h3v8c0 1.1.9 2 2 2h8v2h-.79c-.45 0-.67.54-.35.85l1.79 1.79c.2.2.51.2.71 0l1.79-1.79c.32-.31.09-.85-.35-.85H18v-2h3c.55 0 1-.45 1-1s-.45-1-1-1zm-5-2h2V8c0-1.1-.9-2-2-2h-6v2h5c.55 0 1 .45 1 1v5z"/>
        </SvgIcon>
    }
}
