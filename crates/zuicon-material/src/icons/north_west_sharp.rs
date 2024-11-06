// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NorthWestSharp)]
pub fn north_west_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NorthWestSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,15h2V8.41L18.59,20L20,18.59L8.41,7H15V5H5V15z"/>
        </SvgIcon>
    }
}
