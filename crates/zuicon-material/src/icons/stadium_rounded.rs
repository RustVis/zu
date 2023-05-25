// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StadiumRounded)]
pub fn stadium_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="StadiumRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M6.11,5.45L3.72,6.64C3.39,6.8,3,6.56,3,6.19V3.81C3,3.44,3.39,3.2,3.72,3.36l2.38,1.19C6.47,4.74,6.47,5.26,6.11,5.45z M18,3.81v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C18.39,3.2,18,3.44,18,3.81z M11,2.81 v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C11.39,2.2,11,2.44,11,2.81z M5,10.04 C6.38,10.53,8.77,11,12,11s5.62-0.47,7-0.96C19,9.86,16.22,9,12,9S5,9.86,5,10.04z M14,17h-4c-0.55,0-1,0.45-1,1l0,3.88 C4.94,21.49,2,20.34,2,19v-9c0-1.66,4.48-3,10-3s10,1.34,10,3v9c0,1.34-2.94,2.48-7,2.87L15,18C15,17.45,14.55,17,14,17z"/>
        </SvgIcon>
    }
}
