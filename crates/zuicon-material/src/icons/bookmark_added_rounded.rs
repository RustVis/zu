// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkAddedRounded)]
pub fn bookmark_added_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkAddedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,5c0-1.1,0.9-2,2-2l7,0c-0.63,0.84-1,1.87-1,3c0,2.76,2.24,5,5,5c0.34,0,0.68-0.03,1-0.1v8.58c0,0.72-0.73,1.2-1.39,0.92 L12,18l-5.61,2.4C5.73,20.69,5,20.2,5,19.48V5z M22.07,3.34c0.39,0.39,0.39,1.02,0,1.41l-3.54,3.54c-0.39,0.39-1.02,0.39-1.41,0 l-1.41-1.41c-0.39-0.39-0.39-1.02,0-1.41c0.39-0.39,1.02-0.39,1.41,0l0.71,0.71l2.83-2.83C21.05,2.95,21.68,2.95,22.07,3.34z"/>
        </SvgIcon>
    }
}
