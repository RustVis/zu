// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use web_sys::SubmitEvent;
use yew::{function_component, html, use_state, Callback, Html};
use zu::button::Button;
use zu::button_base::ButtonType;
use zu::form_control::FormControl;
use zu::form_control_label::FormControlLabel;
use zu::form_helper_text::FormHelperText;
use zu::form_label::FormLabel;
use zu::radio::Radio;
use zu::radio_group::RadioGroup;
use zu::styles::button_variant::ButtonVariant;
use zu::styles::color::Color;
use zu::styles::label_variant::LabelVariant;
use zu::styles::position::Position;
use zu::styles::size::Size;

use crate::components::demo_box::DemoBox;

#[function_component(ControlledSection)]
fn controlled_section() -> Html {
    let value = use_state(String::new);
    let handle_change = {
        let value_clone = value.clone();
        Callback::from(move |value: String| {
            log::info!("on changed to {value}");
            value_clone.set(value);
        })
    };

    html! {
        <>
        <h2>{"Controlled"}</h2>
        <p>{"You can control the radio with the value and on_change props:"}</p>
        <DemoBox>
        <FormControl>
            <FormLabel id="demo-controlled-radio-buttons-group">{"Gender"}</FormLabel>
            <RadioGroup
                aria_labelled_by="demo-controlled-radio-buttons-group"
                name="controlled-radio-buttons-group"
                value={value.as_str().to_owned()}
                on_change={handle_change}
            >
                <FormControlLabel value="female" control={html!{<Radio />}} label="Female" />
                <FormControlLabel value="male" control={html!{<Radio />}} label="Male" />
            </RadioGroup>
        </FormControl>
        </DemoBox>
        </>
    }
}

#[function_component(StandaloneSection)]
fn standalone_section() -> Html {
    let value = use_state(String::new);
    let handle_change = {
        let value_clone = value.clone();
        Callback::from(move |value: String| {
            value_clone.set(value);
        })
    };

    html! {
        <>
        <h2>{"Standalone radio buttons"}</h2>
        <p>{"Radio can also be used standalone, without the RadioGroup wrapper."}</p>
        <DemoBox>
        <Radio
            checked={value.as_str() == "a"}
            on_change={handle_change.clone()}
            value="a"
            name="radio-buttons"
            input_aria_label="A"
            />
        <Radio
            checked={value.as_str() == "b"}
            on_change={handle_change}
            value="b"
            name="radio-buttons"
            input_aria_label="B"
        />
        </DemoBox>
        </>
    }
}

#[function_component(SizeSection)]
fn size_section() -> Html {
    let value = use_state(String::new);
    let handle_change = {
        let value_clone = value.clone();
        Callback::from(move |value: String| {
            value_clone.set(value);
        })
    };

    html! {
        <>
        <h2>{"Size"}</h2>
        <p>{"Use the size prop or customize the font size of the svg icons to change the size of the radios."}</p>
        <DemoBox>
        <div>
            <Radio
                size={Size::Small}
                name="size-radio-button-demo"
                input_aria_label="a"
                checked={value.as_str() == "a"}
                on_change={handle_change.clone()}
            />
            <Radio
                size={Size::Medium}
                name="size-radio-button-demo"
                input_aria_label="b"
                checked={value.as_str() == "b"}
                on_change={handle_change.clone()}
            />
            <Radio
                size={Size::Large}
                name="size-radio-button-demo"
                input_aria_label="c"
                checked={value.as_str() == "c"}
                on_change={handle_change}
            />
        </div>
        </DemoBox>
        </>
    }
}

#[function_component(ColorSection)]
fn color_section() -> Html {
    let value = use_state(String::new);
    let handle_change = {
        let value_clone = value.clone();
        Callback::from(move |value: String| {
            value_clone.set(value);
        })
    };
    html! {
        <>
        <h2>{"Color"}</h2>
        <DemoBox>
        <div>
            <Radio
                name="color-radio-button-demo"
                input_aria_label="a"
                checked={value.as_str() == "a"}
                on_change={handle_change.clone()}
            />
            <Radio
                name="color-radio-button-demo"
                input_aria_label="b"
                checked={value.as_str() == "b"}
                on_change={handle_change.clone()}
                color={Color::Secondary}
            />
            <Radio
                name="color-radio-button-demo"
                input_aria_label="c"
                checked={value.as_str() == "c"}
                on_change={handle_change.clone()}
                color={Color::Success}
            />
            <Radio
                name="color-radio-button-demo"
                input_aria_label="d"
                checked={value.as_str() == "d"}
                on_change={handle_change.clone()}
                color={Color::Default}
            />
            <Radio
                name="color-radio-button-demo"
                input_aria_label="e"
                checked={value.as_str() == "e"}
                on_change={handle_change}
                color={Color::Error}
            />
        </div>
        </DemoBox>
        </>
    }
}

#[function_component(LabelPlacementSection)]
fn label_placement_section() -> Html {
    html! {
        <>
        <h2>{"Label placement"}</h2>
        <p>{"You can change the placement of the label with the FormControlLabel component's labelPlacement prop:"}</p>
        <DemoBox>
        <FormControl>
            <FormLabel id="demo-form-control-label-placement">{"Label placement"}</FormLabel>
            <RadioGroup
                row={true}
                aria_labelled_by="demo-form-control-label-placement"
                name="position"
                default_value="top"
            >
                <FormControlLabel
                    value="top"
                    control={html!{<Radio />}}
                    label="Top"
                    label_position={Position::Top}
                />
                <FormControlLabel
                    value="start"
                    control={html!{<Radio />}}
                    label="Start"
                    label_position={Position::Start}
                />
                <FormControlLabel
                    value="bottom"
                    control={html!{<Radio />}}
                    label="Bottom"
                    label_position={Position::Bottom}
                />
                <FormControlLabel
                    value="end"
                    control={html!{<Radio />}}
                    label="End"
                />
            </RadioGroup>
        </FormControl>
        </DemoBox>
        </>
    }
}

#[function_component(ShowErrorSection)]
fn show_error_section() -> Html {
    let error = use_state(|| false);
    let value = use_state(String::new);
    let handle_submit = Callback::from(move |event: SubmitEvent| {
        log::info!("form submitted with value: {event:?}");
        event.prevent_default();
    });
    let handle_radio_change = {
        let value_clone = value.clone();
        Callback::from(move |value: String| {
            value_clone.set(value);
        })
    };
    let helper_text = "";

    html! {
        <>
        <h2>{"Show error"}</h2>
        <p>{"In general, radio buttons should have a value selected by default. If this is not the case, \
        you can display an error if no value is selected when the form is submitted:"}</p>
        <DemoBox>
        <form onsubmit={handle_submit}>
            <FormControl
                style="margin: 3px;"
                error={*error}
                variant={LabelVariant::Standard}
            >
                <FormLabel id="demo-error-radios">{"Pop quiz: Zu is..."}</FormLabel>
                <RadioGroup
                    aria_labelled_by="demo-error-radios"
                    name="quiz"
                    value={value.as_str().to_owned()}
                    on_change={handle_radio_change}
                >
                    <FormControlLabel
                        value="best"
                        control={html!{<Radio />}}
                        label="The best!"
                    />
                    <FormControlLabel
                        value="worst"
                        control={html!{<Radio />}}
                        label="The worst."
                    />
                </RadioGroup>
                <FormHelperText>{helper_text}</FormHelperText>
                <Button
                    style="margin-top: 1px; margin-right: 1px;"
                    button_type={ButtonType::Submit}
                    variant={ButtonVariant::Outlined}
                >
                    {"Check Answer"}
                </Button>
            </FormControl>
        </form>
        </DemoBox>
        </>
    }
}

#[function_component(RadioGroupPage)]
pub fn radio_group_page() -> Html {
    html! {
        <>
        <h1>{"Radio Group"}</h1>
        <p>{"Use radio buttons when the user needs to see all available options. \
        If available options can be collapsed, consider using a Select component because it uses less space."}</p>
        <p>{"Radio buttons should have the most commonly used option selected by default."}</p>

        <h2>{"Radio group"}</h2>
        <p>{"RadioGroup is a helpful wrapper used to group Radio components that provides an easier API, \
        and proper keyboard accessibility to the group."}</p>
        <DemoBox>
        <FormControl>
            <FormLabel id="demo-radio-buttons-group-label">{"Gender"}</FormLabel>
            <RadioGroup
                aria_labelled_by="demo-radio-buttons-group-label"
                default_value="female"
                name="radio-buttons-group">
                <FormControlLabel value="female" control={html!{<Radio />}} label="Female" />
                <FormControlLabel value="male" control={html!{<Radio />}} label="Male" />
                <FormControlLabel value="other" control={html!{<Radio />}} label="Other" />
            </RadioGroup>
        </FormControl>
        </DemoBox>

        <h2>{"Direction"}</h2>
        <p>{"To lay out the buttons horizontally, set the row prop:"}</p>
        <DemoBox>
        <FormControl>
            <FormLabel id="demo-row-radio-buttons-group-label">{"Gender"}</FormLabel>
            <RadioGroup
                row={true}
                aria_labelled_by="demo-row-radio-buttons-group-label"
                name="row-radio-buttons-group">
                <FormControlLabel value="female" control={html!{<Radio />}} label="Female" />
                <FormControlLabel value="male" control={html!{<Radio />}} label="Male" />
                <FormControlLabel value="other" control={html!{<Radio />}} label="Other" />
                <FormControlLabel
                    value="disabled"
                    disabled={true}
                    control={html!{<Radio />}}
                    label="other"
                    />
            </RadioGroup>
        </FormControl>
        </DemoBox>

        <ControlledSection />
        <StandaloneSection />
        <SizeSection />
        <ColorSection />
        <LabelPlacementSection />
        </>
    }
}
