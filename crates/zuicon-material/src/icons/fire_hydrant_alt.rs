// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FireHydrantAlt)]
pub fn fire_hydrant_alt(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="FireHydrantAlt"
            view_box={props.view_box.clone()}
            >
            <path d="M19,11h-1V8h2V6h-2.35C16.83,3.67,14.61,2,12,2S7.17,3.67,6.35,6H4v2h2v3H5c-1.1,0-2,0.9-2,2v2c0,1.1,0.9,2,2,2h1v3H4v2 h16v-2h-2v-3h1c1.1,0,2-0.9,2-2v-2C21,11.9,20.1,11,19,11z M12,17.5c-1.93,0-3.5-1.57-3.5-3.5s1.57-3.5,3.5-3.5s3.5,1.57,3.5,3.5 S13.93,17.5,12,17.5z"/>
        </SvgIcon>
    }
}
