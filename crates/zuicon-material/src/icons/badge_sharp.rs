// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BadgeSharp)]
pub fn badge_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BadgeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,7h-7V2H9v5H2v15h20V7z M9,12c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5 C7.5,12.67,8.17,12,9,12z M12,18H6v-0.43c0-0.6,0.36-1.15,0.92-1.39C7.56,15.9,8.26,15.75,9,15.75s1.44,0.15,2.08,0.43 c0.55,0.24,0.92,0.78,0.92,1.39V18z M13,9h-2V4h2V9z M18,16.5h-4V15h4V16.5z M18,13.5h-4V12h4V13.5z"/>
        </SvgIcon>
    }
}
