// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsInputComponentRounded)]
pub fn settings_input_component_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsInputComponentRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,2c0-0.55-0.45-1-1-1S3,1.45,3,2v4H2C1.45,6,1,6.45,1,7v5h6V7c0-0.55-0.45-1-1-1H5V2z M9,16c0,1.3,0.84,2.4,2,2.82V22 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3.18c1.16-0.41,2-1.51,2-2.82v-2H9V16z M1,16c0,1.3,0.84,2.4,2,2.82V22c0,0.55,0.45,1,1,1 h0c0.55,0,1-0.45,1-1v-3.18C6.16,18.4,7,17.3,7,16v-2H1V16z M21,6V2c0-0.55-0.45-1-1-1s-1,0.45-1,1v4h-1c-0.55,0-1,0.45-1,1v5h6V7 c0-0.55-0.45-1-1-1H21z M13,2c0-0.55-0.45-1-1-1s-1,0.45-1,1v4h-1C9.45,6,9,6.45,9,7v5h6V7c0-0.55-0.45-1-1-1h-1V2z M17,16 c0,1.3,0.84,2.4,2,2.82V22c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3.18c1.16-0.41,2-1.51,2-2.82v-2h-6V16z"/>
        </SvgIcon>
    }
}
