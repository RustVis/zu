// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalConvenienceStoreOutlined)]
pub fn local_convenience_store_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalConvenienceStoreOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 7V4H5v3H2v13h8v-4h4v4h8V7h-3zm1 11h-4v-4H8v4H4V9h3V6h10v3h3v9zM8 8h2v1H8v3h3v-1H9v-1h2V7H8zm7 1h-1V7h-1v3h2v2h1V7h-1z"/>
        </SvgIcon>
    }
}
