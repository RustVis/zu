// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SosSharp)]
pub fn sos_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SosSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.5,7h-7v10h7V7z M13.5,15h-3V9h3V15z M1,15h4v-2H1V7h6v2H3v2h4v6H1V15z M17,15h4v-2h-4V7h6v2h-4v2h4v6h-6V15z"/>
        </SvgIcon>
    }
}
