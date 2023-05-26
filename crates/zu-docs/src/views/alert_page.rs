// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::{
    alert::{Alert, Variant},
    alert_title::AlertTitle,
    styles::severity::Severity,
};
use zuicon_material::Check;

use crate::components::demo_box::DemoBox;

#[function_component(AlertPage)]
pub fn alert_page() -> Html {
    // TODO(Shaohua): Add close button.
    // TODO(Shaohua): Add transitions.
    html! {
        <div class="container">
        <h1>{"Alert"}</h1>

        <h2>{"Basic alerts"}</h2>
        <p>{"The alert offers four severity levels that set a distinctive icon and color."}</p>
        <DemoBox>
            <Alert severity={Severity::Error}>{"This is an error alert — check it out!"}</Alert>
            <Alert severity={Severity::Warning}>{"This is a warning alert — check it out!"}</Alert>
            <Alert severity={Severity::Info}>{"This is an info alert — check it out!"}</Alert>
            <Alert severity={Severity::Success}>{"This is a success alert — check it out!"}</Alert>
        </DemoBox>

        <h2>{"Description"}</h2>
        <p>{"You can use the AlertTitle component to display a formatted title above the content."}</p>
        <DemoBox>
            <Alert severity={Severity::Error}>
                <AlertTitle>{"Error"}</AlertTitle>
                {"This is an error alert — "}
                <strong>{"check it out!"}</strong>
            </Alert>

            <Alert severity={Severity::Warning}>
                <AlertTitle>{"Warning"}</AlertTitle>
                {"This is a warning alert — "}
                <strong>{"check it out!"}</strong>
            </Alert>

            <Alert severity={Severity::Info}>
                <AlertTitle>{"Info"}</AlertTitle>
                {"This is an info alert — "}
                <strong>{"check it out!"}</strong>
            </Alert>

            <Alert severity={Severity::Success}>
                <AlertTitle>{"Success"}</AlertTitle>
                {"This is a success alert — "}
                <strong>{"check it out!"}</strong>
            </Alert>
        </DemoBox>

        <h2>{"Actions"}</h2>
        <p>{"An alert can have an action, such as a close or undo button.
         It is rendered after the message, at the end of the alert."}</p>
        <DemoBox>
            <Alert>{"This is a success alert — check it out!"}</Alert>
            <Alert>{"This is a success alert — check it out!"}</Alert>
        </DemoBox>

        <h2>{"Icons"}</h2>
        <p>{"The icon prop allows you to add an icon to the beginning of the alert component.
         This will override the default icon for the specified severity."}</p>
        <DemoBox>
            <Alert icon={Some(html!{<Check />})} severity={Severity::Success}>
                {"This is a success alert — check it out!"}
            </Alert>

            <Alert icon={None} severity={Severity::Success}>
                {"This is a success alert — check it out!"}
            </Alert>
        </DemoBox>

        <h2>{"Variants"}</h2>
        <p>{"Two additional variants are available – outlined, and filled:"}</p>
        <h3>{"Outlined"}</h3>
        <DemoBox>
            <Alert variant={Variant::Outlined} severity={Severity::Error}>
                {"This is an error alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Outlined} severity={Severity::Warning}>
                {"This is a warning alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Outlined} severity={Severity::Info}>
                {"This is an info alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Outlined} severity={Severity::Success}>
                {"This is a success alert — check it out!"}
            </Alert>
        </DemoBox>

        <h3>{"Filled"}</h3>
        <DemoBox>
            <Alert variant={Variant::Filled} severity={Severity::Error}>
                {"This is an error alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Filled} severity={Severity::Warning}>
                {"This is a warning alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Filled} severity={Severity::Info}>
                {"This is an info alert — check it out!"}
            </Alert>
            <Alert variant={Variant::Filled} severity={Severity::Success}>
                {"This is a success alert — check it out!"}
            </Alert>
        </DemoBox>

        </div>
    }
}
