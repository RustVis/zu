// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThumbDownOffAltSharp)]
pub fn thumb_down_off_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThumbDownOffAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,11.6V16h8.31l-1.12,5.38L9.83,23L17,15.82V3H4.69L1,11.6z M15,5v9.99l-4.34,4.35l0.61-2.93l0.5-2.41H9.31H3v-1.99 L6.01,5H15z"/>
        </SvgIcon>
    }
}
