// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Wifi1BarSharp)]
pub fn wifi_1_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Wifi1BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.53,17.46L12,21l-3.53-3.54C9.37,16.56,10.62,16,12,16S14.63,16.56,15.53,17.46z"/>
        </SvgIcon>
    }
}
