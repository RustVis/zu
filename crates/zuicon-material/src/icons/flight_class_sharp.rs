// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlightClassSharp)]
pub fn flight_class_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlightClassSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,4h-6v9h6V4z M9.5,16H18v2H8L5,8V4h2v4L9.5,16z M8,19h10v2H8V19z"/>
        </SvgIcon>
    }
}
