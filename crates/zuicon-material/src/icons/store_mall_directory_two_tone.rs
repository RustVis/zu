// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StoreMallDirectoryTwoTone)]
pub fn store_mall_directory_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StoreMallDirectoryTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.64 9l-.6 3h13.92l-.6-3z" opacity=".3"/><path d="M4 7l-1 5v2h1v6h10v-6h4v6h2v-6h1v-2l-1-5H4zm8 11H6v-4h6v4zm-6.96-6l.6-3h12.72l.6 3H5.04zM4 4h16v2H4z"/>
        </SvgIcon>
    }
}
