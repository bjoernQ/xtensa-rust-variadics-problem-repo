#include <inttypes.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

void write_log(uint32_t level, const char *tag, const char *log_line, ...);

extern void somethingsomething() {
    write_log(42, "TAG", "FORMAT", 1, 2, 3, 4, 5, 6, 7, 8, 9);
}
