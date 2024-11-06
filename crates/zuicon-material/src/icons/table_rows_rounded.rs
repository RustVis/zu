// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TableRowsRounded)]
pub fn table_rows_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TableRowsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,8H5C3.9,8,3,7.1,3,6v0c0-1.1,0.9-2,2-2h14c1.1,0,2,0.9,2,2v0C21,7.1,20.1,8,19,8z M19,10H5c-1.1,0-2,0.9-2,2v0 c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v0C21,10.9,20.1,10,19,10z M19,16H5c-1.1,0-2,0.9-2,2v0c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v0 C21,16.9,20.1,16,19,16z"/>
        </SvgIcon>
    }
}
