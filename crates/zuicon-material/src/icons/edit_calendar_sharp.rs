// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditCalendarSharp)]
pub fn edit_calendar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditCalendarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,22H3V4h3V2h2v2h8V2h2v2h3v8h-2v-2H5v10h7V22z M22.13,16.99l1.41-1.41l-2.12-2.12l-1.41,1.41L22.13,16.99z M21.42,17.7 l-5.3,5.3H14v-2.12l5.3-5.3L21.42,17.7z"/>
        </SvgIcon>
    }
}
