// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NightShelterRounded)]
pub fn night_shelter_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NightShelterRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.8,3.9l-6,4.5C4.3,8.78,4,9.37,4,10v9c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-9c0-0.63-0.3-1.22-0.8-1.6l-6-4.5 C12.49,3.37,11.51,3.37,10.8,3.9z M9.75,12.5c0.69,0,1.25,0.56,1.25,1.25S10.44,15,9.75,15S8.5,14.44,8.5,13.75S9.06,12.5,9.75,12.5 z M16.5,18L16.5,18c-0.28,0-0.5-0.22-0.5-0.5v-1H8v1C8,17.78,7.78,18,7.5,18h0C7.22,18,7,17.78,7,17.5v-6C7,11.22,7.22,11,7.5,11h0 C7.78,11,8,11.22,8,11.5v4h3.5v-3c0-0.28,0.22-0.5,0.5-0.5h3c1.1,0,2,0.9,2,2v3.5C17,17.78,16.78,18,16.5,18z"/>
        </SvgIcon>
    }
}
