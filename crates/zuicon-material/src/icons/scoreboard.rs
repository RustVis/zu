// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Scoreboard)]
pub fn scoreboard(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Scoreboard"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.5,13.5H16v-3h1.5V13.5z M20,4h-3V2h-2v2H9V2H7v2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6 C22,4.9,21.1,4,20,4z M9.5,11.5c0,0.55-0.45,1-1,1h-2v1h3V15H5v-2.5c0-0.55,0.45-1,1-1h2v-1H5V9h3.5c0.55,0,1,0.45,1,1V11.5z M12.75,18h-1.5v-1.5h1.5V18z M12.75,14.5h-1.5V13h1.5V14.5z M12.75,11h-1.5V9.5h1.5V11z M12.75,7.5h-1.5V6h1.5V7.5z M19,14 c0,0.55-0.45,1-1,1h-2.5c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1H18c0.55,0,1,0.45,1,1V14z"/>
        </SvgIcon>
    }
}
