#include <stdio.h>

size_t fibonacci(size_t n);

int main() {
  size_t n = 40;
  size_t v = fibonacci(n);
  printf("%ld\n", v);
}

size_t fibonacci(size_t n) {
  if (n == 0)
    return 0;
  if (n == 1)
    return 1;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
