// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BackpackSharp)]
pub fn backpack_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BackpackSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,8v14H4V8c0-1.86,1.28-3.41,3-3.86V2h3v2h4V2h3v2.14C18.72,4.59,20,6.14,20,8z M6,12v2h10v2h2v-4H6z"/>
        </SvgIcon>
    }
}
