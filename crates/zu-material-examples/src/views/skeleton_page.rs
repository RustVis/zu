// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use zu_material::skeleton::{Skeleton, Variant};

#[function_component(SkeletonPage)]
pub fn skeleton_page() -> Html {
    html! {
        <div class="container">
            <h2>{ "Usage" }</h2>
            <Skeleton variant={ Variant::Rect } width={ 210 } height={ 118 } />
            <Skeleton width={ 400 } height={ 120 } />
        </div>
    }
}
