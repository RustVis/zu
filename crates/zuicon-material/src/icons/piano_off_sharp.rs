// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PianoOffSharp)]
pub fn piano_off_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PianoOffSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M21.19,21.19L2.81,2.81L1.39,4.22L3,5.83V21h15.17l1.61,1.61L21.19,21.19z M8.25,19H5V7.83l2,2v4.67h1.25V19z M9.75,19v-4.5 H11v-0.67l3.25,3.25V19H9.75z M5.83,3H21v15.17l-2-2V5h-2v9.17l-4-4V5h-2v3.17L5.83,3z"/>
        </SvgIcon>
    }
}
