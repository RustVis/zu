// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NorthWestRounded)]
pub fn north_west_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NorthWestRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M6,15L6,15c0.56,0,1-0.45,1-1V8.41L17.89,19.3c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L8.41,7H14 c0.55,0,1-0.45,1-1V6c0-0.55-0.45-1-1-1H6C5.45,5,5,5.45,5,6V14C5,14.55,5.45,15,6,15z"/>
        </SvgIcon>
    }
}
