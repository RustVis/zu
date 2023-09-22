// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HouseboatSharp)]
pub fn houseboat_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HouseboatSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,16c-1.95,0-2.1,1-3.34,1c-1.19,0-1.42-1-3.33-1C3.38,16,3.24,17,2,17v2c1.9,0,2.17-1,3.35-1c1.19,0,1.42,1,3.33,1 c1.95,0,2.08-1,3.32-1s1.37,1,3.32,1c1.91,0,2.14-1,3.33-1c1.18,0,1.45,1,3.35,1v-2c-1.24,0-1.38-1-3.33-1c-1.91,0-2.14,1-3.33,1 C14.1,17,13.95,16,12,16z M20.34,11.34L18.67,13H17V9.65l1.32,0.97L19.5,9L12,3.5L4.5,9l1.18,1.61L7,9.65V13H5.33l-1.66-1.66 l-1.41,1.41L4.5,15h15l2.25-2.25L20.34,11.34z M13,13h-2v-2h2V13z"/>
        </SvgIcon>
    }
}
