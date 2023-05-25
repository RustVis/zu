// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CreditCardOffRounded)]
pub fn credit_card_off_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CreditCardOffRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M21.19,21.19L2.81,2.81c-0.39-0.39-1.02-0.39-1.41,0l0,0C1,3.2,1,3.83,1.39,4.22l0.84,0.84C2.09,5.34,2.01,5.66,2.01,6L2,18 c0,1.11,0.89,2,2,2h13.17l2.61,2.61c0.39,0.39,1.02,0.39,1.41,0v0C21.58,22.22,21.58,21.58,21.19,21.19z M4,12V8h1.17l4,4H4z M6.83,4H20c1.11,0,2,0.89,2,2v12c0,0.34-0.08,0.66-0.23,0.94L14.83,12H20V8h-9.17L6.83,4z"/>
        </SvgIcon>
    }
}
