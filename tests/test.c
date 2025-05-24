#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

size_t fib(size_t);

int main() {
  char buf[256];
  ssize_t len = read(STDIN_FILENO, buf, 256);
  buf[len] = '\0';

  int n = atoi(buf);
  size_t v = fib(n);
  len = sprintf(buf, "%lu\n", v);
  write(STDOUT_FILENO, buf, len);
}

size_t fib(size_t n) {
  if (n == 0)
    return 0;
  if (n == 1)
    return 1;
  return fib(n - 1) + fib(n - 2);
}
