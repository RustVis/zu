// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PunchClockSharp)]
pub fn punch_clock_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PunchClockSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,6h-3V1H6v5H3v16h18V6z M8,3h8v3H8V3z M12,19c-2.76,0-5-2.24-5-5s2.24-5,5-5c2.76,0,5,2.24,5,5S14.76,19,12,19z"/>
        </SvgIcon>
    }
}
