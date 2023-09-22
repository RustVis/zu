// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewWeekSharp)]
pub fn view_week_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewWeekSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7.33,20H2V4h5.33V20z M22,20V4h-5.33v16H22z M14.67,20V4H9.33v16H14.67z"/>
        </SvgIcon>
    }
}
