// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DraftsSharp)]
pub fn drafts_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DraftsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21.99 6.86L12 1 2 6.86V20h20l-.01-13.14zM12 13L3.74 7.84 12 3l8.26 4.84L12 13z"/>
        </SvgIcon>
    }
}
