// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UsbRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="UsbRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 7h-2c-.55 0-1 .45-1 1v2c0 .55.45 1 1 1v2h-3V5h1c.41 0 .65-.47.4-.8l-2-2.67c-.2-.27-.6-.27-.8 0l-2 2.67c-.25.33-.01.8.4.8h1v8H8v-2.07c.83-.44 1.38-1.36 1.14-2.43-.17-.77-.77-1.4-1.52-1.61C6.15 6.48 4.8 7.59 4.8 9c0 .85.5 1.56 1.2 1.93V13c0 1.1.9 2 2 2h3v3.05c-.86.45-1.39 1.42-1.13 2.49.18.75.79 1.38 1.54 1.58 1.46.39 2.8-.7 2.8-2.12 0-.85-.49-1.58-1.2-1.95V15h3c1.1 0 2-.9 2-2v-2c.55 0 1-.45 1-1V8C19 7.45 18.55 7 18 7z"/>
        </SvgIcon>
    }
}