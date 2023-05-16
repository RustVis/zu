// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AbcSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AbcSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M21,11h-1.5v-0.5h-2v3h2V13H21v2h-5V9h5V11z M8,9v6H6.5v-1.5h-2V15H3V9H8z M6.5,10.5h-2V12h2V10.5z M13.5,12 c0.55,0,1,0.45,1,1v1c0,0.55-0.45,1-1,1h-4V9h4c0.55,0,1,0.45,1,1v1C14.5,11.55,14.05,12,13.5,12z M11,10.5v0.75h2V10.5H11z M13,12.75h-2v0.75h2V12.75z"/>
        </SvgIcon>
    }
}
