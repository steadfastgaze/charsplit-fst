// SPDX-License-Identifier: MIT OR Apache-2.0

//! Web API for the German compound word splitter using WASM.

use crate::Splitter;
use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
extern "C" {
    // Import console.log for debugging
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// WASM-compatible German compound word splitter.
#[wasm_bindgen]
pub struct WebSplitter {
    inner: Splitter,
}

#[wasm_bindgen]
impl WebSplitter {
    /// Create a new splitter from raw FST data bytes.
    /// 
    /// # Arguments
    /// * `suffix_data` - Raw bytes of the suffix.fst file
    /// * `prefix_data` - Raw bytes of the prefix.fst file  
    /// * `infix_data` - Raw bytes of the infix.fst file
    #[wasm_bindgen(constructor)]
    pub fn new(suffix_data: &[u8], prefix_data: &[u8], infix_data: &[u8]) -> Result<WebSplitter, JsValue> {
        match Splitter::from_raw_data(suffix_data, prefix_data, infix_data) {
            Ok(splitter) => Ok(WebSplitter { inner: splitter }),
            Err(e) => Err(JsValue::from_str(&format!("Failed to create splitter: {}", e))),
        }
    }

    /// Split a German compound word into its component parts.
    /// 
    /// # Arguments
    /// * `word` - The compound word to split
    /// 
    /// # Returns
    /// An array of SplitResult objects, each containing score, part1, and part2
    #[wasm_bindgen]
    pub fn split_compound(&self, word: &str) -> JsValue {
        let results = self.inner.split_compound(word);
        
        // Convert to a JavaScript-compatible format
        let js_results = js_sys::Array::new();
        
        for result in results {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"score".into(), &result.score.into()).unwrap();
            js_sys::Reflect::set(&obj, &"part1".into(), &result.part1.into()).unwrap();
            js_sys::Reflect::set(&obj, &"part2".into(), &result.part2.into()).unwrap();
            
            js_results.push(&obj);
        }
        
        js_results.into()
    }
}

/// Convenience function to split a single word without creating a splitter instance.
/// This is useful for simple use cases where you don't need to reuse the splitter.
#[wasm_bindgen]
pub fn split_word_once(
    suffix_data: &[u8], 
    prefix_data: &[u8], 
    infix_data: &[u8], 
    word: &str
) -> Result<JsValue, JsValue> {
    match Splitter::from_raw_data(suffix_data, prefix_data, infix_data) {
        Ok(splitter) => {
            let results = splitter.split_compound(word);
            
            // Convert to a JavaScript-compatible format
            let js_results = js_sys::Array::new();
            
            for result in results {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"score".into(), &result.score.into()).unwrap();
                js_sys::Reflect::set(&obj, &"part1".into(), &result.part1.into()).unwrap();
                js_sys::Reflect::set(&obj, &"part2".into(), &result.part2.into()).unwrap();
                
                js_results.push(&obj);
            }
            
            Ok(js_results.into())
        },
        Err(e) => Err(JsValue::from_str(&format!("Failed to create splitter: {}", e))),
    }
}