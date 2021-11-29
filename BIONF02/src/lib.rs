use cpython::{PyResult, Python, py_module_initializer, py_fn};
use regex::Regex;

fn gc_content(seq: &str) -> f64 {
    return (seq.matches("G").count() + seq.matches("C").count()) as f64 / seq.len() as f64;
}

fn gc_content_py(_py: Python, seq: &str) -> PyResult<f64> {
    let value = gc_content(seq);
    Ok(value)
}

fn gc_content_py_v2(_py: Python, seq: &str) -> PyResult<f64> {
    let re = Regex::new(r"/G|C/gm").unwrap();
    println!("{}", re.find_iter(seq).count());
    let value = re.find_iter(seq).count() as f64 / seq.len() as f64;
    Ok(value)
}

py_module_initializer!(myrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "gc_content", py_fn!(py, gc_content_py(seq: &str)))?;
    m.add(py, "gc_content.__doc__", "My doc string")?;
    m.add(py, "gc_content_v2", py_fn!(py, gc_content_py_v2(seq: &str)))?;
    Ok(())
});
