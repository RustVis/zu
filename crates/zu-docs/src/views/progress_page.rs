// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use gloo_timers::callback::Interval;
use yew::prelude::*;
use zu::circular_progress::CircularProgress;
use zu::circular_progress::Variant as CircularVariant;
use zu::styles::color::ColorVariant;

#[function_component(ProgressPage)]
pub fn progress_page() -> Html {
    let progress = use_state(|| 0);

    {
        let progress_clone = progress.clone();
        use_effect(move || {
            let timer = Interval::new(800, move || {
                let value = *progress_clone;
                if value >= 100 {
                    progress_clone.set(0);
                } else {
                    progress_clone.set(value + 10);
                }
            });
            || {
                timer.cancel();
            }
        });
    }

    html! {
        <div class="container">
            <h1>{"Progress"}</h1>
            <p>{"Progress indicators inform users about the status of ongoing processes, such as loading an app,
             submitting a form, or saving updates."}</p>
            <p>{"The animations of the components rely on CSS as much as possible to work even before the JavaScript is loaded."}</p>

            <h2>{"Circular"}</h2>
            <h3>{"Circular Indeterminate"}</h3>
            <div class="demo-box">
                <CircularProgress />
            </div>

            <h3>{"Circular color"}</h3>
            <div class="demo-box">
                <CircularProgress color={ColorVariant::Secondary} />
                <CircularProgress color={ColorVariant::Success} />
                <CircularProgress color={ColorVariant::Inherit} />
            </div>

            <h3>{"Circular determinate"}</h3>
            <div class="demo-box">
                <CircularProgress variant={CircularVariant::Determinate} value={25} />
                <CircularProgress variant={CircularVariant::Determinate} value={50} />
                <CircularProgress variant={CircularVariant::Determinate} value={75} />
                <CircularProgress variant={CircularVariant::Determinate} value={100} />
                <CircularProgress variant={CircularVariant::Determinate} value={*progress} />
            </div>

            <h3>{"Circular with label"}</h3>
            <div class="demo-box">
                <CircularProgress variant={CircularVariant::Determinate}
                    with_label={true}
                    value={*progress} />
            </div>
        </div>
    }
}
