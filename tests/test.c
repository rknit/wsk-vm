#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int factorial(int);

int main() {
  char buf[256];
  ssize_t len = read(STDIN_FILENO, buf, 256);
  buf[len] = '\0';

  int n = atoi(buf);
  int fac = factorial(n);
  len = sprintf(buf, "%d\n", fac);
  write(STDOUT_FILENO, buf, len);
}

int factorial(int n) {
  if (n <= 1)
    return 1;
  return n * factorial(n - 1);
}
