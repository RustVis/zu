// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CropRotateRounded)]
pub fn crop_rotate_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CropRotateRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M16 9v5h2V8c0-1.1-.9-2-2-2h-6v2h5c.55 0 1 .45 1 1zm3 7H9c-.55 0-1-.45-1-1V5c0-.55-.45-1-1-1s-1 .45-1 1v1H5c-.55 0-1 .45-1 1s.45 1 1 1h1v8c0 1.1.9 2 2 2h8v1c0 .55.45 1 1 1s1-.45 1-1v-1h1c.55 0 1-.45 1-1s-.45-1-1-1zM17.66 1.4C15.99.51 13.83-.11 11.39.04l3.81 3.81 1.33-1.33c3.09 1.46 5.34 4.37 5.89 7.86.06.41.44.69.86.62.41-.06.69-.45.62-.86-.6-3.8-2.96-7-6.24-8.74zM7.47 21.49c-3.09-1.46-5.34-4.37-5.89-7.86-.06-.41-.44-.69-.86-.62-.41.06-.69.45-.62.86.6 3.81 2.96 7.01 6.24 8.75 1.67.89 3.83 1.51 6.27 1.36L8.8 20.16l-1.33 1.33z"/>
        </SvgIcon>
    }
}
