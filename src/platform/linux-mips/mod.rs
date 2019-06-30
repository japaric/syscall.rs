// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for MIPS Linux.

// Clobbers
// See: Section 3-11 of http://refspecs.linux-foundation.org/elf/mipsabi.pdf

// Role of registers
// See: https://www.linux-mips.org/wiki/Syscall

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(mut nr: usize) -> usize {
    let success: usize;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         :
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall1(mut nr: usize, a1: usize) -> usize {
    let success: usize;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall2(mut nr: usize, a1: usize, a2: usize) -> usize {
    let success: usize;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall3(mut nr: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let success: usize;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall4(mut nr: usize, a1: usize, a2: usize, a3: usize, mut a4: usize) -> usize {
    asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall5(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
) -> usize {
    asm!(".set noat
          subu $$29,20
          sw $5, 16($$29)
          syscall
          addiu $$29,20
          .set at"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "r"(a5)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}

#[inline(always)]
pub unsafe fn syscall6(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    asm!(".set noat
          subu $$29,24
          sw $5, 16($$29)
          sw $6, 20($$29)
          syscall
          addiu $$29,24
          .set at"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "r"(a5) "r"(a6)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 {
        nr
    } else {
        -(nr as isize) as usize
    }
}
