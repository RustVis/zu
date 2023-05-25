// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NightlightOutlined)]
pub fn nightlight_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NightlightOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M14,4c0.34,0,0.68,0.02,1.01,0.07C13.1,6.23,12,9.05,12,12s1.1,5.77,3.01,7.93C14.68,19.98,14.34,20,14,20 c-4.41,0-8-3.59-8-8S9.59,4,14,4 M14,2C8.48,2,4,6.48,4,12s4.48,10,10,10c1.82,0,3.53-0.5,5-1.35c-2.99-1.73-5-4.95-5-8.65 s2.01-6.92,5-8.65C17.53,2.5,15.82,2,14,2L14,2z"/>
        </SvgIcon>
    }
}
