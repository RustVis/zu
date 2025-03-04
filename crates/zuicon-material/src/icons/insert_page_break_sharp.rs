// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InsertPageBreakSharp)]
pub fn insert_page_break_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InsertPageBreakSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,8l-6-6H4.01l-0.01,9H20V8z M13,9V3.5L18.5,9H13z"/>
        </SvgIcon>
    }
}
