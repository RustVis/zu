// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiChannelRounded)]
pub fn wifi_channel_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="WifiChannelRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M4.13,21c0.5,0,0.92-0.38,0.99-0.87c0.65-4.89,1.95-9.01,2.88-10c0.91,0.98,2.19,5.01,2.86,9.82 c0.08,0.6,0.59,1.05,1.19,1.05c0.54,0,1.02-0.36,1.16-0.89C13.83,17.73,15.11,15,16,15c0.9,0,2.19,2.83,2.81,5.2 c0.12,0.48,0.56,0.8,1.05,0.8c0.62,0,1.12-0.52,1.09-1.14C20.75,15.89,19.81,3,16,3c-2.51,0-3.77,5.61-4.4,10.57 C10.79,10.66,9.61,8,8,8c-2.92,0-4.41,8.71-4.85,11.87C3.06,20.47,3.53,21,4.13,21z M16,13c-0.99,0-1.82,0.62-2.5,1.5 c0.57-4.77,1.54-8.62,2.5-9.44c0.97,0.81,1.91,4.67,2.49,9.43C17.81,13.62,16.98,13,16,13z"/>
        </SvgIcon>
    }
}
