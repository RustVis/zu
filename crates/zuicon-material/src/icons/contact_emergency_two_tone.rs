// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ContactEmergencyTwoTone)]
pub fn contact_emergency_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ContactEmergencyTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,19h0.08c1.38-2.39,3.96-4,6.92-4s5.54,1.61,6.92,4H22V5H2V19z M15.03,8.15l0.75-1.3l1.47,0.85V6h1.5v1.7 l1.47-0.85l0.75,1.3L19.5,9l1.47,0.85l-0.75,1.3l-1.47-0.85V12h-1.5v-1.7l-1.47,0.85l-0.75-1.3L16.5,9L15.03,8.15z M9,8 c1.65,0,3,1.35,3,3s-1.35,3-3,3s-3-1.35-3-3S7.35,8,9,8z" opacity=".3"/><path d="M9,14c1.65,0,3-1.35,3-3s-1.35-3-3-3s-3,1.35-3,3S7.35,14,9,14z M9,10c0.54,0,1,0.46,1,1s-0.46,1-1,1s-1-0.46-1-1 S8.46,10,9,10z"/><path d="M22,3H2C0.9,3,0,3.9,0,5v14c0,1.1,0.9,2,2,2h20c1.1,0,1.99-0.9,1.99-2L24,5C24,3.9,23.1,3,22,3z M4.54,19 c1.1-1.22,2.69-2,4.46-2s3.36,0.78,4.46,2H4.54z M22,19h-6.08c-1.38-2.39-3.96-4-6.92-4s-5.54,1.61-6.92,4H2V5h20V19z"/>
        </SvgIcon>
    }
}
