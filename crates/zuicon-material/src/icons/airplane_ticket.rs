// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirplaneTicket)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AirplaneTicket"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20.19,4H4C2.9,4,2.01,4.9,2.01,6v4C3.11,10,4,10.9,4,12s-0.89,2-2,2v4c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6 C22,4.9,21.19,4,20.19,4z M17.73,13.3l-8.86,2.36l-1.66-2.88l0.93-0.25l1.26,0.99l2.39-0.64l-2.4-4.16l1.4-0.38l4.01,3.74 l2.44-0.65c0.51-0.14,1.04,0.17,1.18,0.68C18.55,12.62,18.25,13.15,17.73,13.3z"/>
        </SvgIcon>
    }
}
