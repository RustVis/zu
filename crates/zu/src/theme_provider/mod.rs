// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::ops::Deref;
use stylist::{yew::Global, StyleSource};
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

impl ImplicitClone for ThemeKind {}

#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,

    dark: StyleSource,
    light: StyleSource,
}

impl ThemeContext {
    /// ## Panics
    /// Will got panic if failed to parse themes.
    #[must_use]
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        let dark_style = include_str!(env!("DARK_THEME_CSS"));
        let light_style = include_str!(env!("LIGHT_THEME_CSS"));

        let dark = StyleSource::try_from(dark_style).expect("Failed to parse dark theme");
        let light = StyleSource::try_from(light_style).expect("Failed to parse light theme");

        Self { inner, dark, light }
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
    let theme_css = theme_ctx.style().clone();

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            <Global css={theme_css} />
            {for props.children.iter()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
