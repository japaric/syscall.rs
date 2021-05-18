// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! syscall {
    ($nr:ident, u64, $a1:expr)
        => ( ::sc::syscall2(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr)
        => ( ::sc::syscall4(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr)
        => ( ::sc::syscall3(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr)
        => ( ::sc::syscall4(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, u64, $a3:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, $a3:expr)
        => ( ::sc::syscall5(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, u64, $a3:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                0,
                ($a3 >> 32) as usize, $a3 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, u64, $a3:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr)
        => ( ::sc::syscall4(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr)
        => ( ::sc::syscall5(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr)
        => ( ::sc::syscall4(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, u64, $a3:expr, $a4:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, u64, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                0,
                ($a3 >> 32) as usize, $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, u64, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, $a3:expr, $a4:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                0,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, u64, $a3:expr, $a4:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::sc::syscall5(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr, $a4:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr)
        => ( ::sc::syscall5(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize) );

    ($nr:ident, u64, $a1:expr, u64, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                0,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr, $a5:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize,
                $a5 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, $a4:expr, u64, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                0,
                ($a5 >> 32) as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, u64, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr, u64, $a4:expr, $a5:expr)
        => ( ::sc::syscall9(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, u64, :expr, $a3:expr, $a4:expr, u64, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                ($a5 >> 32) as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, u64, $a4:expr, $a5:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                ($a4 >> 32) as usize, $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr, u64, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize,
                0,
                ($a5 >> 32) as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr, u64, $a5:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize,
                ($a5 >> 32) as usize, $a5 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr, $a5:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize,
                $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, u64, $a5:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                ($a5 >> 32) as usize, $a5 as usize) );

    ($nr:ident, u64, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                ($a1 >> 32) as usize, $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize,
                $a6 as usize) );

    ($nr:ident, $a1:expr, u64, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                0,
                ($a2 >> 32) as usize, $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize,
                $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, u64, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                ($a3 >> 32) as usize, $a3 as usize,
                $a4 as usize,
                $a5 as usize,
                $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, u64, $a4:expr, $a5:expr, $a6:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                0,
                ($a4 >> 32) as usize, $a4 as usize,
                $a5 as usize,
                $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, u64, $a5:expr, $a6:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                ($a5 >> 32) as usize, $a5 as usize,
                $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, u64, $a6:expr)
        => ( ::sc::syscall8(
                ::sc::nr::$nr,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize,
                0,
                ($a6 >> 32) as usize, $a6 as usize) );


    ($nr:ident)
        => ( ::sc::syscall0(
                ::sc::nr::$nr) );

    ($nr:ident, $a1:expr)
        => ( ::sc::syscall1(
                ::sc::nr::$nr,
                $a1 as usize) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::sc::syscall2(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::sc::syscall3(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::sc::syscall4(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::sc::syscall5(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::sc::syscall6(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr)
        => ( ::sc::syscall7(
                ::sc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize,
                $a7 as usize) );
}
