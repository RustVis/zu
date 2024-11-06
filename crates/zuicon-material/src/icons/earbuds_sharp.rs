// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EarbudsSharp)]
pub fn earbuds_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EarbudsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.2,3.01C4.44,2.89,3,4.42,3,6.19L3,16c0,2.76,2.24,5,5,5h0c2.76,0,5-2.24,5-5V8c0-1.66,1.34-3,3-3h0c1.66,0,3,1.34,3,3 v7l-0.83,0c-1.61,0-3.06,1.18-3.17,2.79c-0.12,1.69,1.16,3.1,2.8,3.21c1.76,0.12,3.2-1.42,3.2-3.18L21,8c0-2.76-2.24-5-5-5h0 c-2.76,0-5,2.24-5,5v8c0,1.66-1.34,3-3,3l0,0c-1.66,0-3-1.34-3-3V9l0.83,0C7.44,9,8.89,7.82,9,6.21C9.11,4.53,7.83,3.11,6.2,3.01z"/>
        </SvgIcon>
    }
}
