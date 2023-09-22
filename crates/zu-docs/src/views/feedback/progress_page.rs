// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use gloo_timers::callback::Interval;
use yew::prelude::*;
use zu::circular_progress::CircularProgress;
use zu::circular_progress::Variant as CircularVariant;
use zu::linear_progress::LinearProgress;
use zu::linear_progress::Variant as LinearVariant;
use zu::styles::color::Color;

use crate::components::demo_box::DemoBox;

#[function_component(ProgressPage)]
pub fn progress_page() -> Html {
    let progress = use_state(|| 0);
    let buffer = use_state(|| 10);

    {
        let progress_clone = progress.clone();
        let buffer_clone = buffer.clone();
        use_effect(move || {
            let timer = Interval::new(800, move || {
                let progress_value = *progress_clone;
                let buffer_value = *buffer_clone;
                let diff = 10;
                let diff2 = 10;
                if progress_value >= 100 {
                    progress_clone.set(0);
                } else {
                    progress_clone.set(progress_value + diff);
                }
                if buffer_value >= 100 {
                    buffer_clone.set(10);
                } else {
                    buffer_clone.set(buffer_value + diff + diff2);
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
            <DemoBox>
                <CircularProgress />
            </DemoBox>

            <h3>{"Circular color"}</h3>
            <DemoBox>
                <CircularProgress color={Color::Secondary} />
                <CircularProgress color={Color::Success} />
                <CircularProgress color={Color::Inherit} />
            </DemoBox>

            <h3>{"Circular determinate"}</h3>
            <DemoBox>
                <CircularProgress variant={CircularVariant::Determinate} value={25} />
                <CircularProgress variant={CircularVariant::Determinate} value={50} />
                <CircularProgress variant={CircularVariant::Determinate} value={75} />
                <CircularProgress variant={CircularVariant::Determinate} value={100} />
                <CircularProgress variant={CircularVariant::Determinate} value={*progress} />
            </DemoBox>

            <h3>{"Circular with label"}</h3>
            <DemoBox>
                <CircularProgress variant={CircularVariant::Determinate}
                    with_label={true}
                    value={*progress} />
            </DemoBox>

            <h2>{"Linear"}</h2>
            <h3>{"Linear indeterminate"}</h3>
            <DemoBox>
                <LinearProgress />
            </DemoBox>

            <h3>{"Linear color"}</h3>
            <DemoBox>
                <LinearProgress color={Color::Secondary} />
                <LinearProgress color={Color::Success} />
                <LinearProgress color={Color::Inherit} />
            </DemoBox>

            <h3>{"Linear determinate"}</h3>
            <DemoBox>
                   <LinearProgress variant={LinearVariant::Determinate}
                        value={*progress} />
            </DemoBox>

            <h3>{"Linear with label"}</h3>
            <DemoBox>
                   <LinearProgress variant={LinearVariant::Determinate}
                        with_label={true}
                        value={*progress} />
            </DemoBox>

            <h3>{"Linear buffer"}</h3>
            <DemoBox>
                <LinearProgress variant={LinearVariant::Buffer}
                    value={*progress} value_buffer={*buffer} />
            </DemoBox>

        </div>
    }
}
