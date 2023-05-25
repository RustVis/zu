// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WarningAmber)]
pub fn warning_amber(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WarningAmber"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,5.99L19.53,19H4.47L12,5.99 M12,2L1,21h22L12,2L12,2z"/>
        </SvgIcon>
    }
}
