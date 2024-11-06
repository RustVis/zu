// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DifferenceSharp)]
pub fn difference_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DifferenceSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,23H2V7h2v14h14V23z M15,1H6.01L6,19h15V7L15,1z M16.5,15h-6v-2h6V15z M16.5,9h-2v2h-2V9h-2V7h2V5h2v2h2V9z"/>
        </SvgIcon>
    }
}
