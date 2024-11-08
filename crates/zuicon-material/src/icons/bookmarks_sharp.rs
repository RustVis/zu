// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarksSharp)]
pub fn bookmarks_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarksSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 18l2 1V1H7v2h12v15zM17 5H3v18l7-3 7 3V5z"/>
        </SvgIcon>
    }
}
