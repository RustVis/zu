// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlineSeatFlatRounded)]
pub fn airline_seat_flat_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlineSeatFlatRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 11v2H9V9c0-1.1.9-2 2-2h7c2.21 0 4 1.79 4 4zM2 15c0 .55.45 1 1 1h5v1c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-1h5c.55 0 1-.45 1-1s-.45-1-1-1H3c-.55 0-1 .45-1 1zm5.14-2.9c1.16-1.19 1.14-3.08-.04-4.24-1.19-1.16-3.08-1.14-4.24.04-1.16 1.19-1.14 3.08.04 4.24 1.19 1.16 3.08 1.14 4.24-.04z"/>
        </SvgIcon>
    }
}
