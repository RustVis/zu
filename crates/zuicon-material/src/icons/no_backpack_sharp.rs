// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoBackpackSharp)]
pub fn no_backpack_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoBackpackSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.19,21.19L2.81,2.81L1.39,4.22l2.76,2.76C4.06,7.31,4,7.64,4,8v14h15.17l0.61,0.61L21.19,21.19z M6,14v-2h3.17l2,2H6z M6.98,4.15c0.01,0,0.01-0.01,0.02-0.01V2h3v2h4V2h3v2.14c1.72,0.45,3,2,3,3.86v9.17l-2-2V12h-3.17L6.98,4.15z"/>
        </SvgIcon>
    }
}
