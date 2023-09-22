// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UpgradeSharp)]
pub fn upgrade_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UpgradeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,18v2H8v-2H16z M11,7.99V16h2V7.99h3L12,4L8,7.99H11z"/>
        </SvgIcon>
    }
}
