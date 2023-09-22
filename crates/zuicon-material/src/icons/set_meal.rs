// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SetMeal)]
pub fn set_meal(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SetMeal"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.05,17.56L3.08,18.5L3,17l17.98-0.94L21.05,17.56z M21,19.48H3v1.5h18V19.48z M22,5v7c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h16C21.1,3,22,3.9,22,5z M20,6c-1.68,0-3.04,0.98-3.21,2.23C16.15,7.5,14.06,5.5,10.25,5.5 c-4.67,0-6.75,3-6.75,3s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73C16.96,10.02,18.32,11,20,11V6z"/>
        </SvgIcon>
    }
}
