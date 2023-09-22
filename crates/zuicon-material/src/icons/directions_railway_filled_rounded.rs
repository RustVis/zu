// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsRailwayFilledRounded)]
pub fn directions_railway_filled_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsRailwayFilledRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19l-1.21,0.81C6.11,19.93,6,20.13,6,20.35v0 C6,20.71,6.29,21,6.65,21h10.7c0.36,0,0.65-0.29,0.65-0.65v0c0-0.22-0.11-0.42-0.29-0.54L16.5,19c1.93,0,3.5-1.57,3.5-3.5V6 C20,2.5,16.42,2,12,2z M12,16c-0.83,0-1.5-0.67-1.5-1.5S11.17,13,12,13s1.5,0.67,1.5,1.5S12.83,16,12,16z M18,10H6V7h12V10z" enable-background="new"/>
        </SvgIcon>
    }
}
