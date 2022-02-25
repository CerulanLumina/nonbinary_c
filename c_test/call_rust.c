#include "nonbinary.h"

#include <stdio.h>

unsigned char error_data[1024] = { 0 };

void rust_panic(const char * data) {
    puts(data);
    exit(0xff);
}


int main() {
    nonbinary_set_error_handler(rust_panic, error_data, 1024);
    printf("%d\n", test(21, 21));
}
