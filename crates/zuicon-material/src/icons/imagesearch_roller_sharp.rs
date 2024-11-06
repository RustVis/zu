// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ImagesearchRollerSharp)]
pub fn imagesearch_roller_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ImagesearchRollerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2v6H6V6H4v4h10v5h2v8h-6v-8h2v-3H2V4h4V2H20z"/>
        </SvgIcon>
    }
}
