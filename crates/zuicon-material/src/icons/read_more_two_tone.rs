// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReadMoreTwoTone)]
pub fn read_more_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ReadMoreTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            
        </SvgIcon>
    }
}
