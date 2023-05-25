// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DashboardCustomizeRounded)]
pub fn dashboard_customize_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="DashboardCustomizeRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M4,3h6c0.55,0,1,0.45,1,1v6c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1V4C3,3.45,3.45,3,4,3z M14,3h6c0.55,0,1,0.45,1,1v6 c0,0.55-0.45,1-1,1h-6c-0.55,0-1-0.45-1-1V4C13,3.45,13.45,3,14,3z M4,13h6c0.55,0,1,0.45,1,1v6c0,0.55-0.45,1-1,1H4 c-0.55,0-1-0.45-1-1v-6C3,13.45,3.45,13,4,13z M17,13L17,13c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2v-2C18,13.45,17.55,13,17,13z"/>
        </SvgIcon>
    }
}
