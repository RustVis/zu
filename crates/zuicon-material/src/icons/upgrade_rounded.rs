// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UpgradeRounded)]
pub fn upgrade_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UpgradeRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,19L16,19c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h6C15.55,18,16,18.45,16,19z M11,7.99V15 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V7.99h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.78c-0.2-0.19-0.51-0.19-0.71,0L8.86,7.14 C8.54,7.45,8.76,7.99,9.21,7.99H11z"/>
        </SvgIcon>
    }
}
