// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SegmentTwoTone)]
pub fn segment_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SegmentTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M9,18h12v-2H9V18z M3,6v2h18V6H3z M9,13h12v-2H9V13z"/>
        </SvgIcon>
    }
}
