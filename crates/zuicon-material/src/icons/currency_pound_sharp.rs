// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurrencyPoundSharp)]
pub fn currency_pound_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurrencyPoundSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,21c1.93,0,3.62-1.17,4-3l-1.75-0.88C16,18.21,15.33,19,14,19l-4.9,0c0.83-1,1.5-2.34,1.5-4c0-0.35-0.03-0.69-0.08-1 L14,14v-2l-4.18,0C9,10.42,8,9.6,8,8c0-1.93,1.57-3.5,3.5-3.5c1.5,0,2.79,0.95,3.28,2.28L16.63,6c-0.8-2.05-2.79-3.5-5.13-3.5 C8.46,2.5,6,4.96,6,8c0,1.78,0.79,2.9,1.49,4L6,12v2l2.47,0c0.08,0.31,0.13,0.64,0.13,1c0,2.7-2.6,4-2.6,4v2H14z"/>
        </SvgIcon>
    }
}
