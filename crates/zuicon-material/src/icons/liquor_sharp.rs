// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LiquorSharp)]
pub fn liquor_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LiquorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,14c0,1.3,0.84,2.4,2,2.82V20H3v2h6v-2H7v-3.18C8.16,16.4,9,15.3,9,14V6H3V14z M5,8h2v3H5V8z"/><path d="M22,9l-3-1.01V2h-5v6l-3,1.01V22h11V9z M16,4h1v1h-1V4z M13,10.44l3-0.98V7h1v2.46l3,0.98V12h-7V10.44z M20,20h-7v-2h7 V20z"/>
        </SvgIcon>
    }
}
