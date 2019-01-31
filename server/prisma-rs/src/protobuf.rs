use std::mem;

pub mod prisma {
    include!(concat!(env!("OUT_DIR"), "/prisma.rs"));
}

#[repr(C)]
#[no_mangle]
pub struct ProtoBufEnvelope {
    pub data: *mut u8,
    pub len: usize,
}

/*
impl ProtoBuf {
    pub fn into_boxed_ptr(self) -> *mut ProtoBuf {
        Box::into_raw(Box::new(self))
    }
}
*/

impl Drop for ProtoBufEnvelope {
    fn drop(&mut self) {
        if self.len > 0 {
            unsafe {
                drop(Box::from_raw(self.data));
            };
        }
    }
}

impl From<Vec<u8>> for ProtoBufEnvelope {
    fn from(mut v: Vec<u8>) -> Self {
        let len = v.len();
        let data = v.as_mut_ptr();

        mem::forget(v);
        ProtoBufEnvelope { data, len }
    }
}
