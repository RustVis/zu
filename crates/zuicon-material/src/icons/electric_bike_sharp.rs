// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElectricBikeSharp)]
pub fn electric_bike_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElectricBikeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,7h-0.82L16,1h-4v2h2.6l1.46,4h-4.81l-0.36-1H12V4H7v2h1.75l1.82,5H9.9C9.46,8.77,7.59,7.12,5.25,7.01 C2.45,6.87,0,9.2,0,12c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5 C24,9.2,21.8,7,19,7z M7.82,13c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,11h-1.4l-0.73-2H15C14.56,9.58,14.24,10.25,14.1,11z M19,15c-1.68,0-3-1.32-3-3c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64 l1.88-0.68l-0.97-2.67C18.94,9.01,18.97,9,19,9c1.68,0,3,1.32,3,3S20.68,15,19,15z"/>
        </SvgIcon>
    }
}
