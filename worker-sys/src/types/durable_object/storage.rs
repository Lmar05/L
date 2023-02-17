use wasm_bindgen::prelude::*;

use crate::types::DurableObjectTransaction;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object, js_name=DurableObjectStorage)]
    pub type DurableObjectStorage;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=get)]
    pub fn get(this: &DurableObjectStorage, key: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=get)]
    pub fn get_multiple(
        this: &DurableObjectStorage,
        keys: Vec<JsValue>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=put)]
    pub fn put(
        this: &DurableObjectStorage,
        key: &str,
        value: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=put)]
    pub fn put_multiple(
        this: &DurableObjectStorage,
        value: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=delete)]
    pub fn delete(this: &DurableObjectStorage, key: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=delete)]
    pub fn delete_multiple(
        this: &DurableObjectStorage,
        keys: Vec<JsValue>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=deleteAll)]
    pub fn delete_all(this: &DurableObjectStorage) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=list)]
    pub fn list(this: &DurableObjectStorage) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=list)]
    pub fn list_with_options(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=transaction)]
    pub fn transaction(
        this: &DurableObjectStorage,
        closure: &Closure<dyn FnMut(DurableObjectTransaction)>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=getAlarm)]
    pub fn get_alarm(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=setAlarm)]
    pub fn set_alarm(
        this: &DurableObjectStorage,
        scheduled_time: js_sys::Date,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, js_class=DurableObjectStorage, js_name=deleteAlarm)]
    pub fn delete_alarm(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;
}
