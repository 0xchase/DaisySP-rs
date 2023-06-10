use daisysp_sys::*;

pub struct KarplusString {
    inner: daisysp_String,
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

pub struct Pluck {
    inner: daisysp_Pluck,
    trig: f32,
    buffer: Vec<f32>
}

impl Pluck {
    pub fn new() -> Self {
        Self {
            inner: unsafe { std::mem::zeroed() },
            trig: 0.0,
            buffer: Vec::new()
        }
    }

    pub fn init(&mut self, sample_rate: f32, block_size: usize) {
        self.buffer.resize(block_size, 0.0);
        unsafe {
            daisysp_Pluck_Init(&mut self.inner, sample_rate, self.buffer.as_mut_ptr(), block_size as i32, 0);
        }
    }

    pub fn trig(&mut self) {
        self.trig = 1.0;
    }

    pub fn get_amp(&self) -> f32 {
        self.inner.amp_
    }

    pub fn set_amp(&mut self, amp: f32) {
        self.inner.amp_ = amp;
    }

    pub fn get_freq(&self) -> f32 {
        self.inner.freq_
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.inner.freq_ = freq;
    }

    pub fn get_decay(&self) -> f32 {
        self.inner.decay_
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.inner.decay_ = decay;
    }

    pub fn get_damp(&self) -> f32 {
        self.inner.damp_
    }

    pub fn set_damp(&mut self, damp: f32) {
        self.inner.damp_ = damp;
    }

    pub fn gen(&mut self) -> f32 {
        unsafe {
            let f = daisysp_Pluck_Process(&mut self.inner, &mut self.trig);
            self.trig = 0.0;
            f
        }
    }
}
