// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMosaicTwoTone)]
pub fn auto_awesome_mosaic_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMosaicTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,5v14c0,1.1,0.89,2,2,2h6V3H5C3.89,3,3,3.9,3,5z M9,19H5V5h4V19z"/><path d="M19,3h-6v8h8V5C21,3.9,20.1,3,19,3z M19,9h-4V5h4V9z"/><path d="M13,21h6c1.1,0,2-0.9,2-2v-6h-8V21z M15,15h4v4h-4V15z"/>
        </SvgIcon>
    }
}
