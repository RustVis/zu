// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CommentBankSharp)]
pub fn comment_bank_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CommentBankSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,2v20l4-4h16V2H2z M19,13l-2.5-1.5L14,13V5h5V13z"/>
        </SvgIcon>
    }
}
