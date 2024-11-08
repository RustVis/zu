// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FamilyRestroomSharp)]
pub fn family_restroom_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FamilyRestroomSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S16,5.11,16,4z M20,22v-6h2.5l-3-9l-3,0l-1.17,3.5H17V22H20z M12.5,11.5 c0.83,0,1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S11,9.17,11,10S11.67,11.5,12.5,11.5z M5.5,6c1.11,0,2-0.89,2-2s-0.89-2-2-2 s-2,0.89-2,2S4.39,6,5.5,6z M7.5,22v-7H9V7H2v8h1.5v7H7.5z M14,22v-4h1v-5.5h-5V18h1v4H14z"/>
        </SvgIcon>
    }
}
