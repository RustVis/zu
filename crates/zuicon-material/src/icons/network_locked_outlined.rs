// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkLockedOutlined)]
pub fn network_locked_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NetworkLockedOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M22 16v-.5c0-1.38-1.12-2.5-2.5-2.5S17 14.12 17 15.5v.5c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 0h-3v-.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v.5zM18 5.83v5.43c.47-.16.97-.26 1.5-.26.17 0 .33.03.5.05V1L1 20h13v-2H5.83L18 5.83z"/>
        </SvgIcon>
    }
}
