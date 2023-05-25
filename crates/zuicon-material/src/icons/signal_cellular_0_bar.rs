// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellular0Bar)]
pub fn signal_cellular_0_bar(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SignalCellular0Bar"
            view_box={props.view_box.clone()}
            >
            <path d="M20,6.83V20H6.83L20,6.83 M22,2L2,22h20V2L22,2z"/>
        </SvgIcon>
    }
}
