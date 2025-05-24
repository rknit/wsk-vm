#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

size_t fib(size_t);

int main() {
  size_t n;
  scanf("%lu", &n);

  size_t v = fib(n);
  printf("%lu\n", v);
}

size_t fib(size_t n) {
  if (n == 0)
    return 0;
  if (n == 1)
    return 1;
  return fib(n - 1) + fib(n - 2);
}
