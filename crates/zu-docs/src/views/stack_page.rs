// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use stylist::style;
use yew::{function_component, html, Children, Html, Properties};
use zu::paper::Paper;
use zu::stack::Stack;
use zu::styles::direction::Direction;
use zu::styles::spacing::Spacing;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Item)]
pub fn item(props: &Props) -> Html {
    let style = style!(
        r#"
        padding: var(--zu-spacing-xs);
        text-align: center;
        color: var(--zu-palette-text-secondary);
        background-color: white;
    "#
    )
    .unwrap();
    html! {
        <Paper classes={style.get_class_name().to_owned()}>
        {props.children.clone()}
        </Paper>
    }
}

#[function_component(StackPage)]
pub fn stack_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Stack"}</h1>

        <h2>{"Basics"}</h2>
        <p>{"The Stack component acts as a generic container, wrapping around the elements to be arranged."}</p>
        <div class="demo-box">
            <Stack spacing={Spacing::MiddleNudge}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </div>

        <h2>{"Stack v.s. Grid"}</h2>
        <p>{"Stack is concerned with one-dimensional layouts,
          while Grid handles two-dimensional layouts.
          The default direction is column which stacks children vertically."}</p>

        <h2>{"Direction"}</h2>
        <p>{"By default, Stack arranges items vertically in a column.
         Use the direction prop to position items horizontally in a row:"}</p>

        <div class="demo-box">
            <Stack direction={Direction::Row} spacing={Spacing::Small}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </div>

        <h2>{"Dividers"}</h2>
        <p>{"Use the divider prop to insert an element between each child.\
         This works particularly well with the Divider component, as shown below:"}</p>
        <div class="demo-box">
            <Stack direction={Direction::Row} spacing={Spacing::Small}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </div>
        </div>
    }
}