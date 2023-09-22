// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TitleSharp)]
pub fn title_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TitleSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5 4v3h5.5v12h3V7H19V4H5z"/>
        </SvgIcon>
    }
}
