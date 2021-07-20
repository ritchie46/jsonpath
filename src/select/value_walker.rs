use std::collections::HashSet;

use serde_json::Value;

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num(mut vec: Vec<&'a Value>, index: f64) -> Vec<&'a Value> {
        fn _walk_mut<'a>(v: &'a Value, tmp: &mut Vec<&'a Value>, index: f64) {
            if v.is_array() {
                if let Some(vv) = v.get(index as usize) {
                    tmp.push(vv);
                }
            }

            match v {
                Value::Array(vec) => {
                    for v in vec {
                        _walk_mut(v, tmp, index);
                    }
                }
                Value::Object(map) => {
                    for (_, v) in map {
                        _walk_mut(&v, tmp, index);
                    }
                }
                _ => {}
            }
        }

        let len = vec.len();
        for i in 0..len {
            if let Some(value) = vec.get(i) {
                _walk_mut(value, &mut vec, index)
            }
        }
        vec.drain(0..len);
        vec
    }

    pub fn all_with_str(vec: &[&'a Value], tmp: &mut Vec<&'a Value>, key: &str, is_filter: bool) {
        if is_filter {
            Self::walk(vec, tmp, &|v| match v {
                Value::Object(map) if map.contains_key(key) => Some(vec![v]),
                _ => None,
            });
        } else {
            Self::walk(vec, tmp, &|v| match v {
                Value::Object(map) => map.get(key).map(|v| vec![v]),
                _ => None,
            });
        }
    }

    pub fn all(mut vec: Vec<&'a Value>) -> Vec<&'a Value> {
        fn _walk_mut<'a>(v: &'a Value, tmp: &mut Vec<&'a Value>) {
            match v {
                Value::Array(vec) => tmp.extend(vec),
                Value::Object(map) => {
                    tmp.extend(map.values());
                }
                _ => {},
            }

            match v {
                Value::Array(vec) => {
                    for v in vec {
                        _walk_mut(v, tmp);
                    }
                }
                Value::Object(map) => {
                    for (_, v) in map {
                        _walk_mut(&v, tmp);
                    }
                }
                _ => {}
            }
        }

        let len = vec.len();
        for i in 0..len {
            _walk_mut(&vec[i], &mut vec);
        }
        vec.drain(0..len);
        vec
    }

    fn walk<F>(vec: &[&'a Value], tmp: &mut Vec<&'a Value>, fun: &F) where F: Fn(&Value) -> Option<Vec<&Value>> {
        for v in vec {
            Self::_walk(v, tmp, fun);
        }
    }

    fn _walk<F>(v: &'a Value, tmp: &mut Vec<&'a Value>, fun: &F) where F: Fn(&Value) -> Option<Vec<&Value>> {
        if let Some(mut ret) = fun(v) {
            tmp.append(&mut ret);
        }

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

