// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideogameAssetOffSharp)]
pub fn videogame_asset_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideogameAssetOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.83,18H22V6H8.83L20.83,18z M17.5,9c0.83,0,1.5,0.67,1.5,1.5S18.33,12,17.5,12S16,11.33,16,10.5S16.67,9,17.5,9z M21.19,21.19L2.81,2.81L1.39,4.22L3.17,6H2v12h13.17l4.61,4.61L21.19,21.19z M9,13v2H7v-2H5v-2h2V9.83L10.17,13H9z"/>
        </SvgIcon>
    }
}
