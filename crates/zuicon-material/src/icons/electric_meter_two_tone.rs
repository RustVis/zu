// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElectricMeterTwoTone)]
pub fn electric_meter_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ElectricMeterTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M12,4c-3.86,0-7,3.14-7,7s3.14,7,7,7s7-3.14,7-7S15.86,4,12,4z M14.25,14l-3,3l-1.5-1.5L11,14.25L9.75,13 l3-3l1.5,1.5L13,12.75L14.25,14z M16,9H8V7h8V9z" opacity=".3"/><path d="M12,2c-4.97,0-9,4.03-9,9c0,3.92,2.51,7.24,6,8.48V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V22h2v-2.52 c3.49-1.24,6-4.56,6-8.48C21,6.03,16.97,2,12,2z M12,18c-3.86,0-7-3.14-7-7s3.14-7,7-7s7,3.14,7,7S15.86,18,12,18z"/>
        </SvgIcon>
    }
}
