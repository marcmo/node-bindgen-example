use node_bindgen::core::val::JsEnv;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::JSValue;
use node_bindgen::core::NjError;
use node_bindgen::core::TryIntoJs;
use node_bindgen::sys::napi_value;

#[derive(Default, Debug)]
pub struct StreamUpdated {
    pub signature: String,
    pub bytes: i32,
    pub rows: i32,
}

#[derive(Default, Debug)]
pub struct SearchUpdated {
    pub signature: String,
    pub bytes: i32,
    pub rows: i32,
    pub done: f64,
}

#[derive(Debug)]
pub enum Events {
    StreamUpdated(StreamUpdated),
    SearchUpdated(SearchUpdated),
}

impl JSValue<'_> for Events {
    fn convert_to_rust(env: &JsEnv, n_value: napi_value) -> Result<Self, NjError> {
        // check if it is integer
        if let Ok(js_obj) = env.convert_to_rust::<JsObject>(n_value) {
            if let Some(val_property) = js_obj.get_property("signature")? {
                let signature = val_property.as_value::<String>()?;
                match signature.as_str() {
                    "StreamUpdated" => {
                        let mut data = StreamUpdated::default();
                        if let Some(val_property) = js_obj.get_property("bytes")? {
                            data.bytes = val_property.as_value::<i32>()?;
                        } else {
                            return Err(NjError::Other("bytes is not found".to_owned()));
                        }
                        if let Some(val_property) = js_obj.get_property("rows")? {
                            data.rows = val_property.as_value::<i32>()?;
                        } else {
                            return Err(NjError::Other("rows is not found".to_owned()));
                        }
                        Ok(Self::StreamUpdated(data))
                    }
                    "SearchUpdated" => {
                        let mut data = SearchUpdated::default();
                        if let Some(val_property) = js_obj.get_property("bytes")? {
                            data.bytes = val_property.as_value::<i32>()?;
                        } else {
                            return Err(NjError::Other("bytes is not found".to_owned()));
                        }
                        if let Some(val_property) = js_obj.get_property("rows")? {
                            data.rows = val_property.as_value::<i32>()?;
                        } else {
                            return Err(NjError::Other("rows is not found".to_owned()));
                        }
                        if let Some(val_property) = js_obj.get_property("done")? {
                            data.done = val_property.as_value::<f64>()?;
                        } else {
                            return Err(NjError::Other("done is not found".to_owned()));
                        }
                        Ok(Self::SearchUpdated(data))
                    }
                    _ => Err(NjError::Other("Unknown event has been gotten".to_owned())),
                }
            } else {
                Err(NjError::Other("Fail to find event signature".to_owned()))
            }
        } else {
            Err(NjError::Other("not valid format".to_owned()))
        }
    }
}
impl TryIntoJs for Events {
    /// serialize into json object
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::create(js_env)?;
        match self {
            Events::StreamUpdated(data) => {
                json.set_property("signature", "StreamUpdated".to_string().try_to_js(js_env)?)?;
                json.set_property("bytes", data.bytes.try_to_js(js_env)?)?;
                json.set_property("rows", data.rows.try_to_js(js_env)?)?;
                json.try_to_js(js_env)
            }
            Events::SearchUpdated(data) => {
                json.set_property("signature", "SearchUpdated".to_string().try_to_js(js_env)?)?;
                json.set_property("bytes", data.bytes.try_to_js(js_env)?)?;
                json.set_property("rows", data.rows.try_to_js(js_env)?)?;
                json.set_property("done", data.done.try_to_js(js_env)?)?;
                json.try_to_js(js_env)
            }
        }
    }
}
