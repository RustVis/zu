// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhishingOutlined)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PhishingOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M17,6.18V2h-2v4.18C13.84,6.6,13,7.7,13,9s0.84,2.4,2,2.82V15c0,2.21-1.79,4-4,4s-4-1.79-4-4v-1.17l1.59,1.59L10,14L5,9v6 c0,3.31,2.69,6,6,6s6-2.69,6-6v-3.18c1.16-0.41,2-1.51,2-2.82S18.16,6.6,17,6.18z M16,10c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S16.55,10,16,10z"/>
        </SvgIcon>
    }
}