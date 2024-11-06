// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OneKPlusSharp)]
pub fn one_k_plus_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OneKPlusSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M9,15H7.5v-4.5H6V9h3V15z M13.75,15L12,12.75V15h-1.5V9H12v2.25L13.75,9h1.75l-2.25,3l2.25,3H13.75z M19,12.5h-1.5V14h-1v-1.5H15v-1h1.5V10h1v1.5H19V12.5z"/>
        </SvgIcon>
    }
}
