// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PriceChangeSharp)]
pub fn price_change_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PriceChangeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M2,4v16h20V4H2z M12,10H8v1h4v5h-2v1H8v-1H6v-2h4v-1H6V8h2V7h2v1h2V10z M16,16.25l-2-2h4L16,16.25z M14,10l2-2l2,2H14z"/>
        </SvgIcon>
    }
}
