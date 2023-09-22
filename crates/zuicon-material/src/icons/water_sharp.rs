// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WaterSharp)]
pub fn water_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WaterSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21.98,14H22H21.98z M5.35,13c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1 c1.19,0,1.4,0.98,3.31,1v-2c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1 c-1.19,0-1.42-1-3.33-1C3.38,11,3.24,12,2,12v2C3.9,14,4.17,13,5.35,13z M18.67,15c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1 c-1.95,0-2.1,1-3.34,1c-1.24,0-1.38-1-3.33-1c-1.95,0-2.1,1-3.34,1v2c1.95,0,2.11-1,3.34-1c1.24,0,1.38,1,3.33,1 c1.95,0,2.1-1,3.34-1c1.19,0,1.42,1,3.33,1c1.94,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1v-2C20.76,16,20.62,15,18.67,15z M5.35,9 c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.4,0.98,3.31,1V8 c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1C10.04,7,9.9,8,8.66,8C7.47,8,7.24,7,5.33,7 C3.38,7,3.24,8,2,8v2C3.9,10,4.17,9,5.35,9z"/>
        </SvgIcon>
    }
}
