// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PunchClockRounded)]
pub fn punch_clock_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PunchClockRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,6h-1V3c0-1.1-0.9-2-2-2H8C6.9,1,6,1.9,6,3v3H5C3.9,6,3,6.9,3,8v12c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V8 C21,6.9,20.1,6,19,6z M8,3h8v3H8V3z M12,19c-2.76,0-5-2.24-5-5s2.24-5,5-5c2.76,0,5,2.24,5,5S14.76,19,12,19z"/><path d="M12.5,13.79V12c0-0.28-0.22-0.5-0.5-0.5h0c-0.28,0-0.5,0.22-0.5,0.5v2c0,0.13,0.05,0.26,0.15,0.35l1.14,1.14 c0.2,0.2,0.51,0.2,0.71,0c0.2-0.2,0.2-0.51,0-0.71L12.5,13.79z"/>
        </SvgIcon>
    }
}
