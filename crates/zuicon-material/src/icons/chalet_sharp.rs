// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChaletSharp)]
pub fn chalet_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChaletSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,7.5l7.5,7.5l-1.41,1.41L15,15.33V20h-4v-5H9v5H5v-4.67l-1.09,1.09L2.5,15L10,7.5z M22,6.5h-1.19l0.75-0.75l-0.71-0.71 L19.39,6.5H18.5V5.61l1.45-1.45l-0.71-0.71L18.5,4.19V3h-1v1.19l-0.75-0.75l-0.71,0.71l1.45,1.45V6.5h-0.89l-1.45-1.45l-0.71,0.71 l0.75,0.75H14v1h1.19l-0.75,0.75l0.71,0.71l1.45-1.45h0.89v0.89l-1.45,1.45l0.71,0.71l0.75-0.75V11h1V9.81l0.75,0.75l0.71-0.71 L18.5,8.39V7.5h0.89l1.45,1.45l0.71-0.71L20.81,7.5H22V6.5z"/>
        </SvgIcon>
    }
}
