// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EmojiFoodBeverageSharp)]
pub fn emoji_food_beverage_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EmojiFoodBeverageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,3H9v2.4L11,7v5H6V7l2-1.6V3H4v14h14v-7h2c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,8h-2V5h2V8z"/>
        </SvgIcon>
    }
}
