// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AccountTreeRounded)]
pub fn account_tree_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AccountTreeRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,11h3c1.11,0,2-0.9,2-2V5c0-1.11-0.9-2-2-2h-3c-1.11,0-2,0.9-2,2v1H9.01V5c0-1.11-0.9-2-2-2H4C2.9,3,2,3.9,2,5v4 c0,1.11,0.9,2,2,2h3c1.11,0,2-0.9,2-2V8H11v7.01c0,1.65,1.34,2.99,2.99,2.99H15v1c0,1.11,0.9,2,2,2h3c1.11,0,2-0.9,2-2v-4 c0-1.11-0.9-2-2-2h-3c-1.11,0-2,0.9-2,2v1h-1.01C13.45,16,13,15.55,13,15.01V8h2v1C15,10.1,15.9,11,17,11z"/>
        </SvgIcon>
    }
}
