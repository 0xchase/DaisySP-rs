use daisysp_sys::*;

pub struct KarplusString {
    pub inner: daisysp_String,
}

impl KarplusString {
    pub fn new() -> Self {
        Self { inner: unsafe { std::mem::zeroed() } }
    }

    pub fn init(&mut self, sample_rate: f32) {
        unsafe {
            daisysp_String_Init(&mut self.inner, sample_rate);
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            daisysp_String_Reset(&mut self.inner);
        }
    }

    pub fn process(&mut self, in_: f32) -> f32 {
        unsafe { daisysp_String_Process(&mut self.inner, in_) }
    }

    pub fn set_freq(&mut self, freq: f32) {
        unsafe {
            daisysp_String_SetFreq(&mut self.inner, freq);
        }
    }

    pub fn set_nonlinearity(&mut self, amount: f32) {
        unsafe {
            daisysp_String_SetNonLinearity(&mut self.inner, amount);
        }
    }

    pub fn set_brightness(&mut self, amount: f32) {
        unsafe {
            daisysp_String_SetBrightness(&mut self.inner, amount);
        }
    }

    pub fn set_damping(&mut self, amount: f32) {
        unsafe {
            daisysp_String_SetDamping(&mut self.inner, amount);
        }
    }
}
