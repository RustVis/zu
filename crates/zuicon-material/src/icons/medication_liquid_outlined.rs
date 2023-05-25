// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MedicationLiquidOutlined)]
pub fn medication_liquid_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MedicationLiquidOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M2,21h14V6H2V21z M5,12h2.5V9.5h3V12H13v3h-2.5v2.5h-3V15H5V12z"/><path d="M20,6c-1.68,0-3,1.76-3,4c0,1.77,0.83,3.22,2,3.76V21h2v-7.24c1.17-0.54,2-1.99,2-3.76C23,7.76,21.68,6,20,6z"/>
        </SvgIcon>
    }
}
