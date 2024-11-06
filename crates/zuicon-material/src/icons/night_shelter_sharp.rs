// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NightShelterSharp)]
pub fn night_shelter_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NightShelterSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3L4,9v12h16V9L12,3z M9.75,12.5c0.69,0,1.25,0.56,1.25,1.25S10.44,15,9.75,15S8.5,14.44,8.5,13.75S9.06,12.5,9.75,12.5z M17,18h-1v-1.5H8V18H7v-7h1v4.5h3.5V12H17V18z"/>
        </SvgIcon>
    }
}
