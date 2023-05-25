// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BorderColorRounded)]
pub fn border_color_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="BorderColorRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M20,24H4c-1.1,0-2-0.9-2-2v0c0-1.1,0.9-2,2-2h16c1.1,0,2,0.9,2,2v0C22,23.1,21.1,24,20,24z M13.06,5.19l3.75,3.75l-8.77,8.77C7.86,17.9,7.6,18,7.34,18H5c-0.55,0-1-0.45-1-1v-2.34c0-0.27,0.11-0.52,0.29-0.71L13.06,5.19z M17.88,7.87l-3.75-3.75l1.83-1.83c0.39-0.39,1.02-0.39,1.41,0l2.34,2.34c0.39,0.39,0.39,1.02,0,1.41L17.88,7.87z" enable-background="new"/>
        </SvgIcon>
    }
}
