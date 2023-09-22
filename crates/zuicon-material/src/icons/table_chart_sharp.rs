// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TableChartSharp)]
pub fn table_chart_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TableChartSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10 10.02h5V21h-5V10.02zM17 21h5V10h-5v11zm5-18H3v5h19V3zM3 21h5V10H3v11z"/>
        </SvgIcon>
    }
}
