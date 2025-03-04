// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurrencyRuble)]
pub fn currency_ruble(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurrencyRuble"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.5,3H7v9H5v2h2v2H5v2h2v3h2v-3h4v-2H9v-2h4.5c3.04,0,5.5-2.46,5.5-5.5C19,5.46,16.54,3,13.5,3z M13.5,12H9V5h4.5 C15.43,5,17,6.57,17,8.5S15.43,12,13.5,12z"/>
        </SvgIcon>
    }
}
