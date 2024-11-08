// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularNodataRounded)]
pub fn signal_cellular_nodata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularNodataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,13h-7c-1.1,0-2,0.9-2,2v7H4.41c-0.89,0-1.34-1.08-0.71-1.71L20.29,3.71C20.92,3.08,22,3.52,22,4.41V13z M20.3,14.71 L20.3,14.71c-0.39-0.39-1.02-0.39-1.41,0l-1.39,1.39l-1.39-1.39c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41 l1.39,1.39l-1.39,1.39c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0l1.39-1.38l1.39,1.38 c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41l-1.38-1.39l1.38-1.39C20.69,15.73,20.69,15.1,20.3,14.71z"/>
        </SvgIcon>
    }
}
