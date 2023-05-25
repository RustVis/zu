// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AttachmentRounded)]
pub fn attachment_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AttachmentRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17.75 16H7.17c-2.09 0-3.95-1.53-4.15-3.61C2.79 10.01 4.66 8 7 8h12.36c1.31 0 2.5.94 2.63 2.24.15 1.5-1.02 2.76-2.49 2.76H9c-.55 0-1-.45-1-1s.45-1 1-1h8.75c.41 0 .75-.34.75-.75s-.34-.75-.75-.75H9.14c-1.31 0-2.5.94-2.63 2.24-.15 1.5 1.02 2.76 2.49 2.76h10.33c2.09 0 3.95-1.53 4.15-3.61.23-2.39-1.64-4.39-3.98-4.39H7.23c-2.87 0-5.44 2.1-5.71 4.96-.3 3.29 2.26 6.04 5.48 6.04h10.75c.41 0 .75-.34.75-.75s-.34-.75-.75-.75z"/>
        </SvgIcon>
    }
}
