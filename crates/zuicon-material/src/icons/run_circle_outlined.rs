// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RunCircleOutlined)]
pub fn run_circle_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="RunCircleOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8 s3.58-8,8-8s8,3.58,8,8S16.42,20,12,20z"/><path d="M13.54,8.97c-0.23-0.47-0.76-0.71-1.26-0.53L9,9.65V12h1v-1.65l1.54-0.57l-0.96,4.89L7.8,14.1l-0.2,0.98l3.76,0.77 l0.52-2.64L13,14.42V18h1v-3.97l-1.32-1.44l0.41-2.35C13.99,11.46,15.3,12,16,12v-1C15.59,11,14.37,10.67,13.54,8.97z"/>
        </SvgIcon>
    }
}
