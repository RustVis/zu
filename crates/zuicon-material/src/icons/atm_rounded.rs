// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AtmRounded)]
pub fn atm_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AtmRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 9.75c0 .41.34.75.75.75h1.5v3.75c0 .41.34.75.75.75s.75-.34.75-.75V10.5h1.5c.41 0 .75-.34.75-.75S13.66 9 13.25 9h-4.5c-.41 0-.75.34-.75.75zM6 9H3c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75v-.75h2v.75c0 .41.34.75.75.75s.75-.34.75-.75V10c0-.55-.45-1-1-1zm-.5 3h-2v-1.5h2V12zM21 9h-4.5c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75V10.5h1v2.75c0 .41.34.75.75.75s.75-.34.75-.75v-2.76h1v3.76c0 .41.34.75.75.75s.75-.34.75-.75V10c0-.55-.45-1-1-1z"/>
        </SvgIcon>
    }
}
