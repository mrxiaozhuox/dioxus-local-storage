#![allow(non_snake_case)]

mod builder;
mod storage_iter;
mod use_local_storage;

use self::storage_iter::{on_storage, UseStorageData};
pub use self::{
    builder::UseStorageBuilder, storage_iter::StorageIter, use_local_storage::UseLocalStorage,
};
use dioxus::core::ScopeState;
use gloo_events::EventListener;
use log::{info, warn};
use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{window, Storage, StorageEvent, Window};

/// hooks for window's size with config
///
/// # Arguments
///
/// returns: [`WindowSize`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_storage::{use_local_storage};
///
/// fn App(cx: Scope) -> Element {
///     let hook = use_local_storage(&cx);
///
///     cx.render(rsx!(
///         h1 { "Local Storage: {hook:#?}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_local_storage(cx: &ScopeState) -> &UseLocalStorage {
    UseStorageBuilder::default().use_local_storage(cx)
}
