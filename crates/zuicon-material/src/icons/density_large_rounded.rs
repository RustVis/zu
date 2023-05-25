// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DensityLargeRounded)]
pub fn density_large_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="DensityLargeRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M4,5h16c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4C3,4.55,3.45,5,4,5z"/><path d="M20,19H4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1C21,19.45,20.55,19,20,19z"/>
        </SvgIcon>
    }
}
