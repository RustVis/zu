// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsBoatFilledSharp)]
pub fn directions_boat_filled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsBoatFilledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,21c-1.39,0-2.78-0.47-4-1.32c-2.44,1.71-5.56,1.71-8,0C6.78,20.53,5.39,21,4,21H2v2h2c1.38,0,2.74-0.35,4-0.99 c2.52,1.29,5.48,1.29,8,0c1.26,0.65,2.62,0.99,4,0.99h2v-2H20z M3.95,19H4c1.6,0,3.02-0.88,4-2c0.98,1.12,2.4,2,4,2s3.02-0.88,4-2 c0.98,1.12,2.4,2,4,2h0.05l2.18-7.65L20,10.62V4h-5V1H9v3H4v6.62l-2.23,0.73L3.95,19z M6,6h12v3.97L12,8L6,9.97V6z"/>
        </SvgIcon>
    }
}
