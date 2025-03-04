// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookSharp)]
pub fn book_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 2H4v20h16V2zM6 4h5v8l-2.5-1.5L6 12V4z"/>
        </SvgIcon>
    }
}
