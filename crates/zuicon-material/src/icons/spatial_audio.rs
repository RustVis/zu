// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpatialAudio)]
pub fn spatial_audio(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SpatialAudio"
            view_box={props.view_box.clone()}
            >
            <path d="M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z"/><path d="M16,1h-2c0,4.97,4.03,9,9,9V8C19.14,8,16,4.86,16,1z"/><path d="M20,1h-2c0,2.76,2.24,5,5,5V4C21.35,4,20,2.65,20,1z"/>
        </SvgIcon>
    }
}
