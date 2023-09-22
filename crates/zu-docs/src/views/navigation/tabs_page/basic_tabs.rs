// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, use_state, Callback, Children, Html, Properties};
use zu::r#box::Box;
use zu::tab::Tab;
use zu::tabs::Tabs;
use zu::typography::Typography;

#[function_component(BasicTabs)]
pub fn basic_tabs() -> Html {
    let value = use_state(|| 0);
    let handle_change = {
        let value_clone = value.clone();
        Callback::from(move |(new_value, old_value): (i32, i32)| {
            log::info!("value change: {old_value} => {new_value}");
            value_clone.set(new_value);
        })
    };

    html! {
        <>
            <Box style="border-bottom: 1px; border-color: 'divider';">
            <Tabs value={*value} on_change={handle_change} aria_label="basic tabs example">
                <Tab value={0} label={html!{"Item One"}} />
                <Tab value={1} label={html!{"Item Two"}} />
                <Tab value={2} label={html!{"Item Three"}} />
            </Tabs>
            </Box>
            <TabPanel value={*value} index={0}>
                {"Item One"}
            </TabPanel>
            <TabPanel value={*value} index={1}>
                {"Item Two"}
            </TabPanel>
            <TabPanel value={*value} index={2}>
                {"Item Three"}
            </TabPanel>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TabPanelProps {
    pub index: i32,
    pub value: i32,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TabPanel)]
pub fn tab_panel(props: &TabPanelProps) -> Html {
    html! {
        <div
            role="tabpanel"
            hidden={props.value != props.index}
            id={format!("simple-tabpanel-{}", props.index)}
            >
            if props.value == props.index {
                <Box>
                    <Typography>{for props.children.iter()}</Typography>
                </Box>
            }
        </div>
    }
}
