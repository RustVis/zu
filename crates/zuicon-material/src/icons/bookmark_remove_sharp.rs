// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkRemoveSharp)]
pub fn bookmark_remove_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkRemoveSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,7h-6V5h6V7z M19,10.9c-0.64,0.13-1.32,0.14-2.02,0c-1.91-0.38-3.47-1.92-3.87-3.83C12.79,5.54,13.18,4.1,14,3L5,3v18 l7-3l7,3V10.9z"/>
        </SvgIcon>
    }
}
