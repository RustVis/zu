// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkAddedSharp)]
pub fn bookmark_added_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkAddedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,21l-7-3l-7,3V3h9c-0.63,0.84-1,1.87-1,3c0,2.76,2.24,5,5,5c0.34,0,0.68-0.03,1-0.1V21z M17.83,9L15,6.17l1.41-1.41 l1.41,1.41l3.54-3.54l1.41,1.41L17.83,9z"/>
        </SvgIcon>
    }
}
