// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InsertPageBreakTwoTone)]
pub fn insert_page_break_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InsertPageBreakTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,20H6v-3H4v3c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-3h-2V20z"/><path d="M6,4h7v5h5v2h2V8l-6-6H6C4.9,2,4,2.9,4,4v7h2V4z"/>
        </SvgIcon>
    }
}
