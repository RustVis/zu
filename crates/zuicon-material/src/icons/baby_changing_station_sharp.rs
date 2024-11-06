// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BabyChangingStationSharp)]
pub fn baby_changing_station_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BabyChangingStationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,8v2h-3L8.31,8.82L7,12.75V22H3V12l1.58-4.63C4.86,6.53,5.63,6.01,6.46,6C6.74,6,7.02,6.05,7.3,6.18l4.15,1.83L14,8z M8,1C6.9,1,6,1.9,6,3s0.9,2,2,2s2-0.9,2-2S9.1,1,8,1z M9,19h12v-2H9V19z M19.5,16c0.83,0,1.5-0.67,1.5-1.5 c0-0.83-0.67-1.5-1.5-1.5S18,13.67,18,14.5C18,15.33,18.67,16,19.5,16z M13,13v-2H9v2h2v3h6v-5h-2v2H13z"/>
        </SvgIcon>
    }
}
