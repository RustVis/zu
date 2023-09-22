// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SkipPreviousSharp)]
pub fn skip_previous_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SkipPreviousSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M6 6h2v12H6V6zm3.5 6l8.5 6V6l-8.5 6z"/>
        </SvgIcon>
    }
}
