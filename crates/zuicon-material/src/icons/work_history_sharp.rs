// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WorkHistorySharp)]
pub fn work_history_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WorkHistorySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.66,11.13c2-0.37,3.88,0.11,5.34,1.13V6h-6V2H8v4H2v15h9.68c-0.63-1.33-0.87-2.88-0.52-4.51 C11.75,13.79,13.94,11.63,16.66,11.13z M10,4h4v2h-4V4z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M19.65,20.35l-2.15-2.15V15h1v2.79l1.85,1.85 L19.65,20.35z"/>
        </SvgIcon>
    }
}
