// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IosShareTwoTone)]
pub fn ios_share_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="IosShareTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M18,8h-3v2h3v11H6V10h3V8H6c-1.11,0-2,0.89-2,2v11c0,1.1,0.89,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.89,19.1,8,18,8z"/>
        </SvgIcon>
    }
}
