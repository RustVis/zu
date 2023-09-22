// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CalendarViewDaySharp)]
pub fn calendar_view_day_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CalendarViewDaySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 17h18v2H3v-2zm0-7h18v5H3v-5zm0-4h18v2H3V6z"/>
        </SvgIcon>
    }
}
