// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CommentBankTwoTone)]
pub fn comment_bank_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CommentBankTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18l2-2h14V4H4V18z M13,6h5v8l-2.5-1.5L13,14V6z" opacity=".3"/><path d="M20,2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H6l-2,2V4h16V16z"/>
        </SvgIcon>
    }
}
