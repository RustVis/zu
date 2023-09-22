// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ApprovalSharp)]
pub fn approval_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ApprovalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,14v8h16v-8H4z M18,18H6v-2h12V18z M12,2C9.24,2,7,4.24,7,7l5,7l5-7C17,4.24,14.76,2,12,2z"/>
        </SvgIcon>
    }
}
