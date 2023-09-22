// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WatchLaterSharp)]
pub fn watch_later_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WatchLaterSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M16.2,16.2L11,13V7h1.5v5.2l4.5,2.7L16.2,16.2z"/>
        </SvgIcon>
    }
}
