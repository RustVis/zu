// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewDaySharp)]
pub fn view_day_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewDaySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M2 21h19v-3H2v3zM21 8H2v8h19V8zM2 3v3h19V3H2z"/>
        </SvgIcon>
    }
}
