// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BoySharp)]
pub fn boy_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BoySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,7.5c0.97,0,1.75-0.78,1.75-1.75S12.97,4,12,4s-1.75,0.78-1.75,1.75S11.03,7.5,12,7.5z M14,20v-5h1v-4.5 c0-1.1-0.9-2-2-2h-2c-1.1,0-2,0.9-2,2V15h1v5H14z"/>
        </SvgIcon>
    }
}
