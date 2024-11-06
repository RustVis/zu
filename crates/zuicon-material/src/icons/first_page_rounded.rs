// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FirstPageRounded)]
pub fn first_page_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FirstPageRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 0v24H0V0h24z" fill="none" opacity=".87"/><path d="M17.7 15.89L13.82 12l3.89-3.89c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0l-4.59 4.59c-.39.39-.39 1.02 0 1.41l4.59 4.59c.39.39 1.02.39 1.41 0 .38-.38.38-1.02-.01-1.4zM7 6c.55 0 1 .45 1 1v10c0 .55-.45 1-1 1s-1-.45-1-1V7c0-.55.45-1 1-1z"/>
        </SvgIcon>
    }
}
