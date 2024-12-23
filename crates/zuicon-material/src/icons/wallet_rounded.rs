// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WalletRounded)]
pub fn wallet_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WalletRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,4H6C3.79,4,2,5.79,2,8v8c0,2.21,1.79,4,4,4h12c2.21,0,4-1.79,4-4V8C22,5.79,20.21,4,18,4z M16.14,13.77 c-0.24,0.2-0.57,0.28-0.88,0.2L4.15,11.25C4.45,10.52,5.16,10,6,10h12c0.67,0,1.26,0.34,1.63,0.84L16.14,13.77z M6,6h12 c1.1,0,2,0.9,2,2v0.55C19.41,8.21,18.73,8,18,8H6C5.27,8,4.59,8.21,4,8.55V8C4,6.9,4.9,6,6,6z"/>
        </SvgIcon>
    }
}
