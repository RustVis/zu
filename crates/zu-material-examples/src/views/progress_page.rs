// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use zu_material::circular_progress::CircularProgress;
use zu_material::circular_progress::Variant as CircularVariant;
use zu_material::styles::color::ColorVariant;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub progress: i32,
}

#[function_component(ProgressPage)]
pub fn progress_page(props: &Props) -> Html {
    html! {
        <div class="container">
            <h1>{ "Progress" }</h1>
            <p>{ "Progress indicators inform users about the status of ongoing processes, such as loading an app,\
             submitting a form, or saving updates." }</p>
            <p>{ "The animations of the components rely on CSS as much as possible to work even before the JavaScript is loaded." }</p>

            <h2>{ "Circular" }</h2>
            <h3>{ "Circular Indeterminate" }</h3>
            <div class="preview-box">
                <CircularProgress />
            </div>

            <h3>{ "Circular color" }</h3>
            <div class="preview-box">
                <CircularProgress color={ ColorVariant::Secondary } />
                <CircularProgress color={ ColorVariant::Success } />
                <CircularProgress color={ ColorVariant::Inherit } />
            </div>

            <h3>{ "Circular determinate" }</h3>
            <div class="preview-box">
                <CircularProgress variant={ CircularVariant::Determinate } value={ 25 } />
                <CircularProgress variant={ CircularVariant::Determinate } value={ 50 } />
                <CircularProgress variant={ CircularVariant::Determinate } value={ 75 } />
                <CircularProgress variant={ CircularVariant::Determinate } value={ 100 } />
                <CircularProgress variant={ CircularVariant::Determinate } value={ props.progress } />
            </div>

            <h3>{ "Circular with label" }</h3>
            <div class="preview-box">
                <CircularProgress variant={ CircularVariant::Determinate }
                    with_label={ true }
                    value={ props.progress } />
            </div>
        </div>
    }
}
