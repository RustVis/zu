// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BadgeRounded)]
pub fn badge_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BadgeRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M9,12c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5C7.5,12.67,8.17,12,9,12z M12,18H6 v-0.43c0-0.6,0.36-1.15,0.92-1.39C7.56,15.9,8.26,15.75,9,15.75s1.44,0.15,2.08,0.43c0.55,0.24,0.92,0.78,0.92,1.39V18z M13,9h-2V4 h2V9z M17.25,16.5h-2.5c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75h2.5c0.41,0,0.75,0.34,0.75,0.75v0 C18,16.16,17.66,16.5,17.25,16.5z M17.25,13.5h-2.5c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75h2.5 c0.41,0,0.75,0.34,0.75,0.75v0C18,13.16,17.66,13.5,17.25,13.5z"/>
        </SvgIcon>
    }
}
