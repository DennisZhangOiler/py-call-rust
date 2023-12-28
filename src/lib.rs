use numpy::{ndarray::ArrayView3, PyArray3};
use opencv::{prelude::*, videoio, Result};
use pyo3::prelude::*;

trait AsArray {
    fn try_as_array(&self) -> Result<ArrayView3<u8>>;
}
impl AsArray for opencv::core::Mat {
    fn try_as_array(&self) -> Result<ArrayView3<u8>> {
        if !self.is_continuous() {
            panic!("Unable to convert non-continuous Mat")
        }
        let bytes = self.data_bytes()?;
        let size = self.size()?;
        let a =
            ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes).unwrap();
        Ok(a)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_call_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn capture_in_rust<'py>(py: Python<'py>) -> &PyArray3<u8> {
        let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera
        let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
        if !opened {
            panic!("Unable to open default camera!");
        }
        let mut frame = Mat::default();
        cam.read(&mut frame).unwrap();
        PyArray3::from_array(py, &frame.try_as_array().unwrap())
    }
    Ok(())
}
