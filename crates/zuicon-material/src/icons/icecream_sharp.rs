// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IcecreamSharp)]
pub fn icecream_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IcecreamSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.38,6.24C17.79,3.24,15.14,1,12,1S6.21,3.24,5.62,6.24C4.08,6.81,3,8.29,3,10c0,2.21,1.79,4,4,4 c0.12,0,0.23-0.02,0.34-0.02L12.07,23l4.61-9.03C16.79,13.98,16.89,14,17,14c2.21,0,4-1.79,4-4C21,8.29,19.92,6.81,18.38,6.24z M12.05,18.63l-2.73-5.21C10.15,13.79,11.06,14,12,14c0.95,0,1.88-0.21,2.72-0.6L12.05,18.63z"/>
        </SvgIcon>
    }
}
