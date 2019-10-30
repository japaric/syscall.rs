// Copyright 2019 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for riscv Linux.

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1) "{x11}"(a2)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1) "{x11}"(a2) "{x12}"(a3)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize)
                       -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1) "{x11}"(a2) "{x12}"(a3) "{x13}"(a4)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize)
                       -> usize {
    let ret: usize;
    asm!("ecall" : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1) "{x11}"(a2) "{x12}"(a3) "{x13}"(a4) "{x14}"(a5)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize,
                       a6: usize)
                       -> usize {
    let ret: usize;
    asm!("ecall"
         : "={x10}"(ret)
         : "{x17}"(n) "{x10}"(a1) "{x11}"(a2) "{x12}"(a3) "{x13}"(a4) "{x14}"(a5)
           "{x15}"(a6)
         : "memory" "cc"
         : "volatile");
    ret
}
