// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SkipNext)]
pub fn skip_next(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SkipNext"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/>
        </SvgIcon>
    }
}
