// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CheckBoxOutlineBlankSharp)]
pub fn check_box_outline_blank_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CheckBoxOutlineBlankSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 5v14H5V5h14m2-2H3v18h18V3z"/>
        </SvgIcon>
    }
}
