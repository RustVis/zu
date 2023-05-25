// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NorthEastSharp)]
pub fn north_east_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NorthEastSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M9,5v2h6.59L4,18.59L5.41,20L17,8.41V15h2V5H9z"/>
        </SvgIcon>
    }
}
