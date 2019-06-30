// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for PowerPC Linux.

// Reference: https://www.kernel.org/doc/Documentation/powerpc/syscall64-abi.txt

// Clobbers
// See Section 3-14 of https://refspecs.linux-foundation.org/elf/elfspec_ppc.pdf

#![allow(unused_assignments)]
#![allow(unused_variables)]

pub mod nr;

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> usize {
    let ret: usize;
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "={r3}"(ret)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    ret
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall1(mut n: usize, mut a1: usize) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall2(mut n: usize, mut a1: usize, mut a2: usize) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2)
         :
         : "cr0" "memory" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall3(mut n: usize, mut a1: usize, mut a2: usize, mut a3: usize) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3)
         :
         : "cr0" "memory" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall4(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
         :
         : "cr0" "memory" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall5(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5)
         :
         : "cr0" "memory" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall6(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
    mut a6: usize,
) -> usize {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5) "+{r8}"(a6)
         :
         : "cr0" "memory" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}
