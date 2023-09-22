// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkAddedOutlined)]
pub fn bookmark_added_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkAddedOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,11v6.97l-5-2.14l-5,2.14V5h6V3H7C5.9,3,5,3.9,5,5v16l7-3l7,3V11H17z M17.83,9L15,6.17l1.41-1.41l1.41,1.41l3.54-3.54 l1.41,1.41L17.83,9z"/>
        </SvgIcon>
    }
}
