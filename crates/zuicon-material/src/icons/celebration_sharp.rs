// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CelebrationSharp)]
pub fn celebration_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CelebrationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.53,12.53L21,6.05l1.48,1.48l1.06-1.06L21,3.93l-7.53,7.53L14.53,12.53z"/><path d="M10.94,6L9.47,7.47l1.06,1.06l2.54-2.54l-2.54-2.53L9.47,4.53L10.94,6z"/><path d="M18.97,9.97l-3.5,3.5l1.06,1.06L19,12.06l2.5,2.49l1.06-1.06L18.97,9.97z"/><path d="M15.97,4.97l-4.5,4.5l1.06,1.06L18.07,5l-3.53-3.53l-1.06,1.06L15.97,4.97z"/>
        </SvgIcon>
    }
}
