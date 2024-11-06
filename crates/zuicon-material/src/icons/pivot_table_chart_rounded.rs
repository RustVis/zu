// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PivotTableChartRounded)]
pub fn pivot_table_chart_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PivotTableChartRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21,5c0-1.1-0.9-2-2-2h-9v5h11V5z"/><path d="M3,19c0,1.1,0.9,2,2,2h3V10H3V19z"/><path d="M3,5v3h5V3H5C3.9,3,3,3.9,3,5z"/><path d="M17.65,9.35l-2.79,2.79C14.54,12.46,14.76,13,15.21,13H17v2c0,1.1-0.9,2-2,2h-2v-1.79c0-0.45-0.54-0.67-0.85-0.35 l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79c0.31,0.31,0.85,0.09,0.85-0.35V19h2c2.21,0,4-1.79,4-4v-2h1.79 c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.79C18.16,9.16,17.84,9.16,17.65,9.35z"/>
        </SvgIcon>
    }
}
