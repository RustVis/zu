// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RecommendSharp)]
pub fn recommend_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RecommendSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm6 10.05L15.46 18H7v-7.56L12 5l1 1v.53L12.41 10H18v2.05z"/>
        </SvgIcon>
    }
}
