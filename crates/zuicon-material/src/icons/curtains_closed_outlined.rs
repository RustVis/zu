// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsClosedOutlined)]
pub fn curtains_closed_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CurtainsClosedOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M20,19V3H4v16H2v2h20v-2H20z M13,5v14h-2V5H13z M6,5h3v14H6V5z M15,19V5h3v14H15z"/>
        </SvgIcon>
    }
}
