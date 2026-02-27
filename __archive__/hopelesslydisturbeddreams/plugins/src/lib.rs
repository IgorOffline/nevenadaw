use std::collections::HashMap;
use std::fmt::Display;
use std::pin::Pin;
use std::sync::Arc;

pub use serde_json::from_str as deserialize_field;
pub use serde_json::to_string as serialize_field;

pub fn hello() -> String {
    "Hello".to_string()
}

// --- START: RANGE ---

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T: Default> Default for Range<T> {
    fn default() -> Self {
        Self {
            min: T::default(),
            max: T::default(),
        }
    }
}

impl Range<f32> {
    pub fn normalize(&self, value: f32) -> f32 {
        let span = self.max - self.min;
        if span == 0.0 {
            0.0
        } else {
            (value - self.min) / span
        }
    }

    pub fn unnormalize(&self, normalized: f32) -> f32 {
        self.min + (self.max - self.min) * normalized
    }
}

impl Range<i32> {
    pub fn normalize(&self, value: i32) -> f32 {
        let span = self.max - self.min;
        if span == 0 {
            0.0
        } else {
            (value - self.min) as f32 / span as f32
        }
    }

    pub fn unnormalize(&self, normalized: f32) -> i32 {
        (self.min as f32 + (self.max - self.min) as f32 * normalized).round() as i32
    }
}

// --- END: RANGE ---

//
// --- --- ---
//
// PARAM
//
// --- --- ---
//

pub trait Param {
    type Plain;
    fn set_from_string(&mut self, string: &str) -> bool;
    fn plain_value(&self) -> Self::Plain;
    fn set_plain_value(&mut self, plain: Self::Plain);
    fn normalized_value(&self) -> f32;
    fn set_normalized_value(&mut self, normalized: f32);
    fn normalized_value_to_string(&self, normalized: f32, include_unit: bool) -> String;
    fn string_to_normalized_value(&self, string: &str) -> Option<f32>;
    fn as_ptr(&self) -> ParamPtr;
}

pub struct PlainParam<T> {
    pub value: T,
    pub value_changed: Option<Arc<dyn Fn(T) -> () + Send + Sync>>,
    pub range: Range<T>,
    pub name: &'static str,
    pub unit: &'static str,
    pub value_to_string: Option<Arc<dyn Fn(T) -> String + Send + Sync>>,
    pub string_to_value: Option<Arc<dyn Fn(&str) -> Option<T> + Send + Sync>>,
}

pub type FloatParam = PlainParam<f32>;
pub type IntParam = PlainParam<i32>;

pub struct BoolParam {
    pub value: bool,
    pub value_changed: Option<Arc<dyn Fn(bool) -> () + Send + Sync>>,
    pub name: &'static str,
    pub value_to_string: Option<Arc<dyn Fn(bool) -> String + Send + Sync>>,
    pub string_to_value: Option<Arc<dyn Fn(&str) -> Option<bool> + Send + Sync>>,
}

impl<T> Default for PlainParam<T>
where
    T: Default,
    Range<T>: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            value_changed: None,
            range: Range::default(),
            name: "",
            unit: "",
            value_to_string: None,
            string_to_value: None,
        }
    }
}

impl Default for BoolParam {
    fn default() -> Self {
        Self {
            value: false,
            value_changed: None,
            name: "",
            value_to_string: None,
            string_to_value: None,
        }
    }
}

macro_rules! impl_plainparam {
    ($ty:ident, $plain:ty) => {
        impl Param for $ty {
            type Plain = $plain;

            fn set_from_string(&mut self, string: &str) -> bool {
                let value = match &self.string_to_value {
                    Some(f) => f(string),
                    // TODO: Check how Rust's parse function handles trailing garbage
                    None => string.parse().ok(),
                };

                match value {
                    Some(plain) => {
                        self.value = plain;
                        true
                    }
                    None => false,
                }
            }

            fn plain_value(&self) -> Self::Plain {
                self.value
            }

            fn set_plain_value(&mut self, plain: Self::Plain) {
                self.value = plain;
                if let Some(f) = &self.value_changed {
                    f(plain);
                }
            }

            fn normalized_value(&self) -> f32 {
                self.range.normalize(self.value)
            }

            fn set_normalized_value(&mut self, normalized: f32) {
                self.set_plain_value(self.range.unnormalize(normalized));
            }

            fn normalized_value_to_string(&self, normalized: f32, include_unit: bool) -> String {
                let value = self.range.unnormalize(normalized);
                match (&self.value_to_string, include_unit) {
                    (Some(f), true) => format!("{}{}", f(value), self.unit),
                    (Some(f), false) => format!("{}", f(value)),
                    (None, true) => format!("{}{}", value, self.unit),
                    (None, false) => format!("{}", value),
                }
            }

            fn string_to_normalized_value(&self, string: &str) -> Option<f32> {
                let value = match &self.string_to_value {
                    Some(f) => f(string),
                    // TODO: Check how Rust's parse function handles trailing garbage
                    None => string.parse().ok(),
                }?;

                Some(self.range.normalize(value))
            }

            fn as_ptr(&self) -> ParamPtr {
                ParamPtr::$ty(self as *const $ty as *mut $ty)
            }
        }
    };
}

impl_plainparam!(FloatParam, f32);
impl_plainparam!(IntParam, i32);

impl Param for BoolParam {
    type Plain = bool;

    fn set_from_string(&mut self, string: &str) -> bool {
        let value = match &self.string_to_value {
            Some(f) => f(string),
            None => Some(string.eq_ignore_ascii_case("true") || string.eq_ignore_ascii_case("on")),
        };

        match value {
            Some(plain) => {
                self.value = plain;
                true
            }
            None => false,
        }
    }

    fn plain_value(&self) -> Self::Plain {
        self.value
    }

    fn set_plain_value(&mut self, plain: Self::Plain) {
        self.value = plain;
        if let Some(f) = &self.value_changed {
            f(plain);
        }
    }

    fn normalized_value(&self) -> f32 {
        if self.value { 1.0 } else { 0.0 }
    }

    fn set_normalized_value(&mut self, normalized: f32) {
        self.set_plain_value(normalized > 0.5);
    }

    fn normalized_value_to_string(&self, normalized: f32, _include_unit: bool) -> String {
        let value = normalized > 0.5;
        match (value, &self.value_to_string) {
            (v, Some(f)) => format!("{}", f(v)),
            (true, None) => String::from("On"),
            (false, None) => String::from("Off"),
        }
    }

    fn string_to_normalized_value(&self, string: &str) -> Option<f32> {
        let value = match &self.string_to_value {
            Some(f) => f(string),
            None => Some(string.eq_ignore_ascii_case("true") || string.eq_ignore_ascii_case("on")),
        }?;

        Some(if value { 1.0 } else { 0.0 })
    }

    fn as_ptr(&self) -> ParamPtr {
        ParamPtr::BoolParam(self as *const BoolParam as *mut BoolParam)
    }
}

impl<T: Display + Copy> Display for PlainParam<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value_to_string {
            Some(func) => write!(f, "{}{}", func(self.value), self.unit),
            None => write!(f, "{}{}", self.value, self.unit),
        }
    }
}

impl Display for BoolParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.value, &self.value_to_string) {
            (v, Some(func)) => write!(f, "{}", func(v)),
            (true, None) => write!(f, "On"),
            (false, None) => write!(f, "Off"),
        }
    }
}

//
// --- --- ---
//
// PARAM INTERNALS
//
// --- --- ---
//

pub trait Params {
    fn param_map(self: Pin<&Self>) -> HashMap<&'static str, ParamPtr>;
    fn param_ids(self: Pin<&Self>) -> &'static [&'static str];
    fn serialize_fields(&self) -> HashMap<String, String>;
    fn deserialize_fields(&self, serialized: &HashMap<String, String>);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParamPtr {
    FloatParam(*mut FloatParam),
    IntParam(*mut IntParam),
    BoolParam(*mut BoolParam),
}

unsafe impl Send for ParamPtr {}
unsafe impl Sync for ParamPtr {}

pub trait PersistentField<'a, T>: Send + Sync
where
    T: serde::Serialize + serde::Deserialize<'a>,
{
    fn set(&self, new_value: T);
    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&T) -> R;
}

impl ParamPtr {
    pub unsafe fn name(&self) -> &'static str {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).name },
            ParamPtr::IntParam(p) => unsafe { (**p).name },
            ParamPtr::BoolParam(p) => unsafe { (**p).name },
        }
    }

    pub unsafe fn unit(&self) -> &'static str {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).unit },
            ParamPtr::IntParam(p) => unsafe { (**p).unit },
            ParamPtr::BoolParam(_) => "",
        }
    }

    pub unsafe fn set_from_string(&mut self, string: &str) -> bool {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).set_from_string(string) },
            ParamPtr::IntParam(p) => unsafe { (**p).set_from_string(string) },
            ParamPtr::BoolParam(p) => unsafe { (**p).set_from_string(string) },
        }
    }

    pub unsafe fn normalized_value(&self) -> f32 {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).normalized_value() },
            ParamPtr::IntParam(p) => unsafe { (**p).normalized_value() },
            ParamPtr::BoolParam(p) => unsafe { (**p).normalized_value() },
        }
    }

    pub unsafe fn set_normalized_value(&self, normalized: f32) {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).set_normalized_value(normalized) },
            ParamPtr::IntParam(p) => unsafe { (**p).set_normalized_value(normalized) },
            ParamPtr::BoolParam(p) => unsafe { (**p).set_normalized_value(normalized) },
        }
    }

    pub unsafe fn preview_normalized(&self, plain: f32) -> f32 {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).range.normalize(plain) },
            ParamPtr::IntParam(p) => unsafe { (**p).range.normalize(plain as i32) },
            ParamPtr::BoolParam(_) => plain,
        }
    }

    pub unsafe fn preview_plain(&self, normalized: f32) -> f32 {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).range.unnormalize(normalized) },
            ParamPtr::IntParam(p) => unsafe { (**p).range.unnormalize(normalized) as f32 },
            ParamPtr::BoolParam(_) => normalized,
        }
    }

    pub unsafe fn normalized_value_to_string(&self, normalized: f32, include_unit: bool) -> String {
        match &self {
            ParamPtr::FloatParam(p) => unsafe {
                (**p).normalized_value_to_string(normalized, include_unit)
            },
            ParamPtr::IntParam(p) => unsafe {
                (**p).normalized_value_to_string(normalized, include_unit)
            },
            ParamPtr::BoolParam(p) => unsafe {
                (**p).normalized_value_to_string(normalized, include_unit)
            },
        }
    }

    pub unsafe fn string_to_normalized_value(&self, string: &str) -> Option<f32> {
        match &self {
            ParamPtr::FloatParam(p) => unsafe { (**p).string_to_normalized_value(string) },
            ParamPtr::IntParam(p) => unsafe { (**p).string_to_normalized_value(string) },
            ParamPtr::BoolParam(p) => unsafe { (**p).string_to_normalized_value(string) },
        }
    }
}

impl<'a, T> PersistentField<'a, T> for std::sync::RwLock<T>
where
    T: serde::Serialize + serde::Deserialize<'a> + Send + Sync,
{
    fn set(&self, new_value: T) {
        *self.write().expect("Poisoned RwLock on write") = new_value;
    }
    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&T) -> R,
    {
        f(&self.read().expect("Poisoned RwLock on read"))
    }
}

impl<'a, T> PersistentField<'a, T> for parking_lot::RwLock<T>
where
    T: serde::Serialize + serde::Deserialize<'a> + Send + Sync,
{
    fn set(&self, new_value: T) {
        *self.write() = new_value;
    }
    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&T) -> R,
    {
        f(&self.read())
    }
}

impl<'a, T> PersistentField<'a, T> for std::sync::Mutex<T>
where
    T: serde::Serialize + serde::Deserialize<'a> + Send + Sync,
{
    fn set(&self, new_value: T) {
        *self.lock().expect("Poisoned Mutex") = new_value;
    }
    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&T) -> R,
    {
        f(&self.lock().expect("Poisoned Mutex"))
    }
}

macro_rules! impl_persistent_field_parking_lot_mutex {
    ($ty:ty) => {
        impl<'a, T> PersistentField<'a, T> for $ty
        where
            T: serde::Serialize + serde::Deserialize<'a> + Send + Sync,
        {
            fn set(&self, new_value: T) {
                *self.lock() = new_value;
            }
            fn map<F, R>(&self, f: F) -> R
            where
                F: Fn(&T) -> R,
            {
                f(&self.lock())
            }
        }
    };
}

impl_persistent_field_parking_lot_mutex!(parking_lot::Mutex<T>);
impl_persistent_field_parking_lot_mutex!(parking_lot::FairMutex<T>);

//
// --- --- ---
//
// UTIL
//
// --- --- ---
//

pub const MINUS_INFINITY_DB: f32 = -100.0;

pub fn db_to_gain(dbs: f32) -> f32 {
    if dbs > MINUS_INFINITY_DB {
        10.0f32.powf(dbs * 0.05)
    } else {
        0.0
    }
}

pub fn gain_to_db(gain: f32) -> f32 {
    if gain > 0.0 {
        gain.log10() * 20.0
    } else {
        MINUS_INFINITY_DB
    }
}

#[allow(dead_code)]
pub(crate) trait ThreadSpawnUnchecked {
    unsafe fn spawn_unchecked_2<F, R>(self, f: F) -> std::io::Result<std::thread::JoinHandle<R>>
    where
        F: FnOnce() -> R + Send,
        R: 'static + Send;
}

impl ThreadSpawnUnchecked for std::thread::Builder {
    unsafe fn spawn_unchecked_2<F, R>(self, f: F) -> std::io::Result<std::thread::JoinHandle<R>>
    where
        F: FnOnce() -> R + Send,
        R: 'static + Send,
    {
        let f: Box<dyn Send + FnOnce() -> R> = Box::new(f);
        // ^ No need for `'_` inside function bodies
        // v but this is more readable
        let _: &Box<dyn '_ + Send + FnOnce() -> R> = &f;
        // Safety: same-layout since only a lifetime difference
        let f: Box<dyn 'static + Send + FnOnce() -> R> = unsafe { std::mem::transmute(f) };
        self.spawn(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_spawn_unchecked_builder() {
        let result = Arc::new(Mutex::new(0));
        let result_clone = result.clone();

        let handle = unsafe {
            std::thread::Builder::new()
                .name("test-thread".into())
                .spawn_unchecked_2(move || {
                    let mut guard = result_clone.lock().unwrap();
                    *guard = 42;
                })
                .expect("failed to spawn thread")
        };

        handle.join().expect("thread panicked");

        assert_eq!(*result.lock().unwrap(), 42);
    }
}

//
// --- --- ---
//
// DEBUG
//
// --- --- ---
//

#[macro_export]
macro_rules! nih_log {
    ($format:expr $(, $($args:tt)*)?) => (
        eprintln!(concat!("[", file!(), ":", line!(), "] ", $format), $($($args)*)?)
    );
}

#[macro_export]
macro_rules! nih_debug_assert {
    ($cond:expr $(,)?) => (
        if cfg!(debug_assertions) && !$cond {
            nih_log!(concat!("Debug assertion failed: ", stringify!($cond)));
        }
    );
    ($cond:expr, $format:expr $(, $($args:tt)*)?) => (
        if cfg!(debug_assertions) && !$cond {
            nih_log!(concat!("Debug assertion failed: ", stringify!($cond), ", ", $format), $($($args)*)?);
        }
    );
}

#[macro_export]
macro_rules! nih_debug_assert_failure {
    () => (
        if cfg!(debug_assertions) {
            nih_log!("Debug assertion failed");
        }
    );
    ($format:expr $(, $($args:tt)*)?) => (
        if cfg!(debug_assertions) {
            nih_log!(concat!("Debug assertion failed: ", $format), $($($args)*)?);
        }
    );
}

#[macro_export]
macro_rules! nih_debug_assert_eq {
    ($left:expr, $right:expr $(,)?) => (
        if cfg!(debug_assertions) && $left != $right {
            nih_log!(concat!("Debug assertion failed: ", stringify!($left), " != ", stringify!($right)));
        }
    );
    (left:expr, $right:expr, $format:expr $(, $($args:tt)*)?) => (
        if cfg!(debug_assertions) && $left != $right  {
            nih_log!(concat!("Debug assertion failed: ", stringify!($left), " != ", stringify!($right), ", ", $format), $($($args)*)?);
        }
    );
}

#[macro_export]
macro_rules! nih_debug_assert_neq {
    ($left:expr, $right:expr $(,)?) => (
        if cfg!(debug_assertions) && $left == $right {
            nih_log!(concat!("Debug assertion failed: ", stringify!($left), " == ", stringify!($right)));
        }
    );
    (left:expr, $right:expr, $format:expr $(, $($args:tt)*)?) => (
        if cfg!(debug_assertions) && $left == $right  {
            nih_log!(concat!("Debug assertion failed: ", stringify!($left), " == ", stringify!($right), ", ", $format), $($($args)*)?);
        }
    );
}

//
// --- --- ---
//
// FORMATTERS
//
// --- --- ---
//

pub fn f32_rounded(digits: usize) -> Option<Arc<dyn Fn(f32) -> String + Send + Sync>> {
    Some(Arc::new(move |x| format!("{:.digits$}", x)))
}

//
// --- --- ---
//
// SINE EXAMPLE
//
// --- --- ---
//
