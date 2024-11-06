// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Filter9PlusSharp)]
pub fn filter_9_plus_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Filter9PlusSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 5H1v18h18v-2H3V5zm11 9V6H9v5h3v1H9v2h5zm-3-5V8h1v1h-1zm12-8H5v18h18V1zm-2 8h-2V7h-2v2h-2v2h2v2h2v-2h2v6H7V3h14v6z"/>
        </SvgIcon>
    }
}
