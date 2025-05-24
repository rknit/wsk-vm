#include <unistd.h>

const char str[] = "Hello World!\n";
const int len = sizeof(str);

int main() { write(1, str, len); }
