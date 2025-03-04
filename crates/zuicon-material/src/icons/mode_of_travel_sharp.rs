// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeOfTravelSharp)]
pub fn mode_of_travel_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeOfTravelSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.31,18.9c-0.96,1-2.06,2.03-3.31,3.1c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2c4,0,7.64,2.92,7.97,7.5l3.53,0L19,14 l-4.5-4.5l3.47,0C17.65,6.24,15.13,4,12,4c-3.35,0-6,2.57-6,6.2c0,2.34,1.95,5.44,6,9.14c0.64-0.59,1.23-1.16,1.77-1.71 c-0.17-0.34-0.27-0.72-0.27-1.12c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5S17.38,19,16,19C15.76,19,15.53,18.97,15.31,18.9z"/>
        </SvgIcon>
    }
}
