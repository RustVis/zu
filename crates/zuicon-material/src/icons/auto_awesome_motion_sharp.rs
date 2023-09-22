// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMotionSharp)]
pub fn auto_awesome_motion_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMotionSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H2v12h2V4h10V2z M18,6H6v12h2V8h10V6z M22,10H10v12h12V10z"/>
        </SvgIcon>
    }
}
