// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TypeSpecimenSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TypeSpecimenSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M22,2H6v16h16V2z M16.63,14.5l-0.8-2.3h-3.63l-0.82,2.3H9.81l3.38-9h1.61l3.38,9H16.63z"/>
        </SvgIcon>
    }
}
