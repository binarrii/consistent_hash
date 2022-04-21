use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use std::os::raw::{c_int, c_long};

struct LinearCongruentialGenerator {
    state: i64,
}

impl LinearCongruentialGenerator {
    pub fn next_double(&mut self) -> f64 {
        self.state = 2862933555777941757i64.wrapping_mul(self.state) + 1;
        ((((((self.state as u64) >> 33) as i64) as i32) + 1) as f64) / (2.0f64).powi(31)
    }
}

#[no_mangle]
pub extern "C" fn consistent_hash(input: c_long, buckets: c_int) -> c_int {
    let mut generator = LinearCongruentialGenerator { state: input };

    let mut candidate: i32 = 0;
    let mut next: i32;
    loop {
        next = (((candidate + 1) as f64) / generator.next_double()) as i32;
        if next >= 0 && next < buckets {
            candidate = next;
        } else {
            return candidate;
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_cc_binarii_hashing_Boot_00024LibHashJNI_consistentHash(
    _env: JNIEnv,
    _class: JClass,
    input: jlong,
    buckets: jint,
) -> jint {
    consistent_hash(input, buckets)
}
