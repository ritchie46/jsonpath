use std::collections::HashSet;

use serde_json::Value;

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num(vec: Vec<&'a Value>, index: f64) -> Vec<&'a Value> {
        Self::walk(vec, &|v, tmp| {
            if v.is_array() {
                if let Some(vv) = v.get(index as usize) {
                    tmp.push(vv);
                }
            }
        })
    }

    pub fn all_with_str(vec: Vec<&'a Value>, key: &str, is_filter: bool) -> Vec<&'a Value> {
        Self::walk(vec, &|v, tmp| {
            if is_filter {
                match v {
                    Value::Object(map) => {
                        if let Some(v) = map.get(key) {
                            tmp.push(v);
                        }
                    },
                    _ => {},
                }
            } else {
                match v {
                    Value::Object(map) => {
                        if let Some(v) = map.get(key) {
                            tmp.push(v);
                        }
                    },
                    _ => {},
                }
            }
        })
    }

    pub fn all(vec: Vec<&'a Value>) -> Vec<&'a Value> {
        Self::walk(vec, &|v, tmp| {
            match v {
                Value::Array(ay) => tmp.extend(ay),
                Value::Object(map) => {
                    tmp.extend(map.values());
                }
                _ => {},
            }
        })
    }

    fn walk<F>(mut vec: Vec<&'a Value>, fun: &F) -> Vec<&'a Value>
        where F: Fn(&'a Value, &mut Vec<&'a Value>) 
    {
        let len = vec.len();
        for i in 0..len {
            Self::_walk(&vec[i], &mut vec, fun);
        }
        vec.drain(0..len);
        vec
    }

    fn _walk<F>(v: &'a Value, tmp: &mut Vec<&'a Value>, fun: &F) 
        where F: Fn(&'a Value, &mut Vec<&'a Value>)
    {
        
        fun(v, tmp);

        match v {
            Value::Array(vec) => {
                for v in vec {
                    Self::_walk(v, tmp, fun);
                }
            }
            Value::Object(map) => {
                for (_, v) in map {
                    Self::_walk(&v, tmp, fun);
                }
            }
            _ => {}
        }
    }

    pub fn walk_dedup(v: &'a Value,
                      tmp: &mut Vec<&'a Value>,
                      key: &str,
                      visited: &mut HashSet<*const Value>, ) {
        match v {
            Value::Object(map) => {
                if map.contains_key(key) {
                    let ptr = v as *const Value;
                    if !visited.contains(&ptr) {
                        visited.insert(ptr);
                        tmp.push(v)
                    }
                }
            }
            Value::Array(vec) => {
                for v in vec {
                    Self::walk_dedup(v, tmp, key, visited);
                }
            }
            _ => {}
        }
    }
}

