long syscall0(long n) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3");
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "=r"(r3)::"memory", "cr0", "r4", "r5", "r6",
                         "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long syscall1(long n, long a1) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3)::"memory", "cr0", "r4", "r5", "r6",
                         "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long syscall2(long n, long a1, long a2) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  register long r4 __asm__("r4") = a2;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4)::"memory", "cr0", "r5",
                         "r6", "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long syscall3(long n, long a1, long a2, long a3) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  register long r4 __asm__("r4") = a2;
  register long r5 __asm__("r5") = a3;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5)::"memory",
                         "cr0", "r6", "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long syscall4(long n, long a1, long a2, long a3, long a4) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  register long r4 __asm__("r4") = a2;
  register long r5 __asm__("r5") = a3;
  register long r6 __asm__("r6") = a4;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5),
                         "+r"(r6)::"memory", "cr0", "r7", "r8", "r9", "r10",
                         "r11", "r12");
  return r3;
}

long syscall5(long n, long a1, long a2, long a3, long a4, long a5) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  register long r4 __asm__("r4") = a2;
  register long r5 __asm__("r5") = a3;
  register long r6 __asm__("r6") = a4;
  register long r7 __asm__("r7") = a5;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5), "+r"(r6),
                         "+r"(r7)::"memory", "cr0", "r8", "r9", "r10", "r11",
                         "r12");
  return r3;
}

long syscall6(long n, long a1, long a2, long a3, long a4, long a5, long a6) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a1;
  register long r4 __asm__("r4") = a2;
  register long r5 __asm__("r5") = a3;
  register long r6 __asm__("r6") = a4;
  register long r7 __asm__("r7") = a5;
  register long r8 __asm__("r8") = a6;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5), "+r"(r6),
                         "+r"(r7), "+r"(r8)::"memory", "cr0", "r9", "r10",
                         "r11", "r12");
  return r3;
}
