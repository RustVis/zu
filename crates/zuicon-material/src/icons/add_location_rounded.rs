// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddLocationRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AddLocationRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M13,7c0-0.55-0.44-1-1-1c-0.55,0-1,0.44-1,1v2H9c-0.55,0-1,0.44-1,1c0,0.55,0.44,1,1,1h2v2 c0,0.55,0.44,1,1,1c0.55,0,1-0.44,1-1v-2h2c0.55,0,1-0.44,1-1c0-0.55-0.44-1-1-1h-2V7z M12,2c4.2,0,8,3.22,8,8.2 c0,3.18-2.45,6.92-7.34,11.23c-0.38,0.33-0.95,0.33-1.33,0C6.45,17.12,4,13.38,4,10.2C4,5.22,7.8,2,12,2z"/>
        </SvgIcon>
    }
}