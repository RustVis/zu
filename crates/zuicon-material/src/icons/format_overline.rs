// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatOverline)]
pub fn format_overline(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatOverline"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3v2H5V3H19z M12,7c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C19,10.13,15.87,7,12,7z M12,18.5 c-2.49,0-4.5-2.01-4.5-4.5S9.51,9.5,12,9.5s4.5,2.01,4.5,4.5S14.49,18.5,12,18.5z"/>
        </SvgIcon>
    }
}
