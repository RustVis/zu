// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShuffleTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ShuffleTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 4h-5.5l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5zM5.41 4L4 5.41l5.17 5.17 1.42-1.41zM20 20v-5.5l-2.04 2.04-3.13-3.13-1.41 1.41 3.13 3.13L14.5 20z"/>
        </SvgIcon>
    }
}