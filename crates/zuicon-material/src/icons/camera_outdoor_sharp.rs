// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraOutdoorSharp)]
pub fn camera_outdoor_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CameraOutdoorSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,14v-2h-6v6h6v-2l2,1.06v-4.12L18,14z M12,3L4,9v12h16v-2H6v-9l6-4.5l6,4.5v1h2V9L12,3z"/>
        </SvgIcon>
    }
}
