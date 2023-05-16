// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AnalyticsSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AnalyticsSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M3,3v18h18V3H3z M9,17H7v-5h2V17z M13,17h-2v-3h2V17z M13,12h-2v-2h2V12z M17,17h-2V7h2V17z"/>
        </SvgIcon>
    }
}
