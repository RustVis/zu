// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IronSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="IronSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M18,6v8h-1V7H7v3h2V9h6v2H6c-2.21,0-4,1.79-4,4v3h15v-2h3V8h2V6H18z"/>
        </SvgIcon>
    }
}
