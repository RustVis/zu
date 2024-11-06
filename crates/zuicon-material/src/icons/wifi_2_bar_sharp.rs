// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Wifi2BarSharp)]
pub fn wifi_2_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Wifi2BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,10c3.03,0,5.78,1.23,7.76,3.22l-2.12,2.12C16.2,13.9,14.2,13,12,13c-2.2,0-4.2,0.9-5.64,2.35l-2.12-2.12 C6.22,11.23,8.97,10,12,10z M12,16c-1.38,0-2.63,0.56-3.53,1.46L12,21l3.53-3.54C14.63,16.56,13.38,16,12,16z"/>
        </SvgIcon>
    }
}
