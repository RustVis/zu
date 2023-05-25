// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoCellSharp)]
pub fn no_cell_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoCellSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.19,21.19L2.81,2.81L1.39,4.22L5,7.83V23h14v-1.17l0.78,0.78L21.19,21.19z M7,18V9.83L15.17,18H7z M8.83,6L5,2.17V1h14 v15.17l-2-2V6H8.83z"/>
        </SvgIcon>
    }
}
