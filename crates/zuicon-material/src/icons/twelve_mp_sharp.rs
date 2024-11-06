// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TwelveMpSharp)]
pub fn twelve_mp_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TwelveMpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M12,8h3V7h-3V5.5h4.5V9h-3v1h3v1.5H12V8z M7,5.5h3v6H8.5V7H7V5.5z M12.5,18.5H11V14h-1v3H8.5v-3h-1v4.5H6 v-6h6.5V18.5z M18,17h-3v1.5h-1.5v-6H18V17z"/>
        </SvgIcon>
    }
}
