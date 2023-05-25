// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OilBarrelSharp)]
pub fn oil_barrel_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="OilBarrelSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M21,13v-2h-2V5h2V3H3v2h2v6H3v2h2v6H3v2h18v-2h-2v-6H21z M12,16c-1.66,0-3-1.32-3-2.95c0-1.3,0.52-1.67,3-4.55 c2.47,2.86,3,3.24,3,4.55C15,14.68,13.66,16,12,16z"/>
        </SvgIcon>
    }
}
