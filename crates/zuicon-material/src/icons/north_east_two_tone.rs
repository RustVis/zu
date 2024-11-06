// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NorthEastTwoTone)]
pub fn north_east_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NorthEastTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,5v2h6.59L4,18.59L5.41,20L17,8.41V15h2V5H9z"/>
        </SvgIcon>
    }
}
