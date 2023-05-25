// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbShadeRounded)]
pub fn wb_shade_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="WbShadeRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M14,14.13L14,14.13c0,0.23,0.09,0.46,0.26,0.63l4.98,4.98c0.17,0.17,0.39,0.26,0.62,0.26h0c0.79,0,1.18-0.95,0.62-1.51 l-4.98-4.98C14.95,12.95,14,13.35,14,14.13z M15,20h2l-3-3v2C14,19.55,14.45,20,15,20z M7.65,4.35L2.85,9.15 C2.54,9.46,2.76,10,3.21,10H4v9c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-9h0.79c0.45,0,0.67-0.54,0.35-0.85L8.35,4.35 C8.16,4.16,7.84,4.16,7.65,4.35z M9,14H7v-4h2V14z"/>
        </SvgIcon>
    }
}
