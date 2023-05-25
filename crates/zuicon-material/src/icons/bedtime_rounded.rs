// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BedtimeRounded)]
pub fn bedtime_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="BedtimeRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M11.65 3.46c.27-.71-.36-1.45-1.12-1.34-5.52.8-9.47 6.07-8.34 11.88.78 4.02 4.09 7.21 8.14 7.87 3.74.61 7.16-.87 9.32-3.44.48-.57.19-1.48-.55-1.62-6.02-1.15-9.68-7.54-7.45-13.35z"/>
        </SvgIcon>
    }
}
