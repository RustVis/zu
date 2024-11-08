// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddchartSharp)]
pub fn addchart_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddchartSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,9h2v8h-2V9z M9,17v-6H7v6H9z M19,19H5V5h6V3H3v18h18v-8h-2V19z M15,13v4h2v-4H15z M19,5V2h-2v3h-3v2h3v3h2V7h3V5H19z"/>
        </SvgIcon>
    }
}
