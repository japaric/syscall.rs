// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for SPARC64 Linux.

// Reference:
// http://math-atlas.sourceforge.net/devel/assembly/abi_sysV_sparc.pdf

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> usize {
    let ret;
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "={o0}"(ret)
         : "{g1}"(nr)
         : "cc" "memory"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(nr: usize, mut a1: usize) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall2(nr: usize, mut a1: usize, a2: usize) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall3(nr: usize, mut a1: usize, a2: usize, a3: usize) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall4(nr: usize, mut a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall5(
    nr: usize,
    mut a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4) "{o4}"(a5)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall6(
    nr: usize,
    mut a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4) "{o4}"(a5)
           "{o5}"(a6)
         : "cc" "memory"
         : "volatile");
    a1
}
