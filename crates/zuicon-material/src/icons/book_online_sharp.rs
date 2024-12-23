// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookOnlineSharp)]
pub fn book_online_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookOnlineSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,1H5v22h14V1z M7,18V6h10v12H7z M16,11l0-3H8l0,3.1c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1L8,16h8v-3c-0.55,0-1-0.45-1-1 C15,11.45,15.45,11,16,11z M12.5,14.5h-1v-1h1V14.5z M12.5,12.5h-1v-1h1V12.5z M12.5,10.5h-1v-1h1V10.5z"/>
        </SvgIcon>
    }
}
