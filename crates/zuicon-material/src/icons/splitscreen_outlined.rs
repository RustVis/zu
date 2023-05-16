// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SplitscreenOutlined)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SplitscreenOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,4v5H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v5c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,15v5H6v-5H18 M18,13H6c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-5C20,13.9,19.1,13,18,13z"/>
        </SvgIcon>
    }
}