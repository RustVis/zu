// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutletSharp)]
pub fn outlet_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutletSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M8,12V7h2v5H8z M14,18h-4l0-1.89 c0-1,0.68-1.92,1.66-2.08C12.92,13.82,14,14.79,14,16V18z M16,12h-2V7h2V12z"/>
        </SvgIcon>
    }
}
