// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PivotTableChartTwoTone)]
pub fn pivot_table_chart_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PivotTableChartTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21,5c0-1.1-0.9-2-2-2h-9v5h11V5z"/><path d="M3,19c0,1.1,0.9,2,2,2h3V10H3V19z"/><path d="M3,5v3h5V3H5C3.9,3,3,3.9,3,5z"/><path d="M18,9l-4,4h3v2c0,1.1-0.9,2-2,2h-2v-3l-4,4l4,4v-3h2c2.21,0,4-1.79,4-4v-2h3L18,9z"/>
        </SvgIcon>
    }
}
