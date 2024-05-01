//! Implementation for WASI
use core::{
    mem::MaybeUninit
};
use crate::Error;

mod intrinsics {
    extern "C" {
        pub fn tapos_block_prefix() -> i32;
    }
}

pub fn tapos_block_prefix() -> u32 {
    return unsafe { intrinsics::tapos_block_prefix() as u32 };
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    let block_prefix = tapos_block_prefix();
    let bytes = block_prefix.to_le_bytes();

    // Copy the bytes into the destination slice.
    for (i, byte) in bytes.iter().enumerate() {
        dest[i] = MaybeUninit::new(*byte);
    }

    Ok(())
    // unsafe { random_get(dest.as_mut_ptr() as *mut u8, dest.len()) }.map_err(|e| {
    //     // The WASI errno will always be non-zero, but we check just in case.
    //     match NonZeroU16::new(e.raw()) {
    //         Some(r) => Error::from(NonZeroU32::from(r)),
    //         None => Error::ERRNO_NOT_POSITIVE,
    //     }
    // })
}
