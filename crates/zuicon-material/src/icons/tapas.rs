// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Tapas)]
pub fn tapas(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Tapas"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,10V1h-8v9c0,1.86,1.28,3.41,3,3.86V21h-2v2h6v-2h-2v-7.14C20.72,13.41,22,11.86,22,10z M20,3v3h-4V3H20z M12.5,11.5 c0,1.38-1.12,2.5-2.5,2.5H8v9H6v-9H4c-1.38,0-2.5-1.12-2.5-2.5C1.5,10.12,2.62,9,4,9h2V8H4C2.62,8,1.5,6.88,1.5,5.5 C1.5,4.12,2.62,3,4,3h2V1h2v2h2c1.38,0,2.5,1.12,2.5,2.5C12.5,6.88,11.38,8,10,8H8v1h2C11.38,9,12.5,10.12,12.5,11.5z"/>
        </SvgIcon>
    }
}
