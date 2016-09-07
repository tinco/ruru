use ruby_sys::hash;

use binding::fixnum;
use types::{CallbackPtr, CallbackMutPtr, Value};
use Object;

pub fn new() -> Value {
    unsafe { hash::rb_hash_new() }
}

pub fn aref(hash: Value, key: Value) -> Value {
    unsafe { hash::rb_hash_aref(hash, key) }
}

pub fn aset(hash: Value, key: Value, value: Value) -> Value {
    unsafe { hash::rb_hash_aset(hash, key, value) }
}

pub fn length(hash: Value) -> i64 {
    unsafe {
        let size = hash::rb_hash_size(hash);

        fixnum::num_to_int(size)
    }
}

pub fn each<K, V, F>(hash: Value, closure_callback: F)
    where K: Object,
          V: Object,
          F: FnMut(K, V)
{
    let closure_ptr = &closure_callback as *const _ as CallbackMutPtr;

    unsafe {
        hash::rb_hash_foreach(hash, hash_callback::<K, V, F> as CallbackPtr, closure_ptr);
    }
}

pub extern "C" fn hash_callback<K, V, F>(key: K, value: V, closure: CallbackMutPtr)
    where K: Object,
          V: Object,
          F: FnMut(K, V)
{
    let closure = closure as *mut F;

    unsafe {
        (*closure)(key, value);
    }
}
