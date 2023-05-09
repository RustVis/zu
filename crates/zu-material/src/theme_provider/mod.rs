// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::ops::Deref;
use stylist::{css, yew::Global, StyleSource};
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    Light,
    Dark,
}

impl Default for ThemeKind {
    fn default() -> Self {
        Self::Light
    }
}

impl ImplicitClone for ThemeKind {}

#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,

    dark: StyleSource,
    light: StyleSource,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self {
            inner,
            dark: css!(""),
            light: css!(""),
        }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind);
    }

    #[must_use]
    pub fn kind(&self) -> ThemeKind {
        *self.inner
    }

    #[must_use]
    pub fn style(&self) -> &StyleSource {
        match self.kind() {
            ThemeKind::Light => &self.light,
            ThemeKind::Dark => &self.dark,
        }
    }
}

impl Deref for ThemeContext {
    type Target = StyleSource;

    fn deref(&self) -> &Self::Target {
        match self.kind() {
            ThemeKind::Light => &self.light,
            ThemeKind::Dark => &self.dark,
        }
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(ThemeKind::default);

    let theme_ctx = ThemeContext::new(theme_kind);
    let global_css = theme_ctx.style().clone();

    html! {
        <ContextProvider<ThemeContext> context={ theme_ctx }>
            <Global css={ global_css } />
            { props.children.clone() }
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
