// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsClosedSharp)]
pub fn curtains_closed_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CurtainsClosedSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M20,19V3H4v16H2v2h20v-2H20z M11,5h2v14h-2V5z"/>
        </SvgIcon>
    }
}
