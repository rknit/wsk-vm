#include <stdio.h>

size_t get_n_prime(size_t n);

int main() {
  size_t n = 300000;
  size_t v = get_n_prime(n);
  printf("prime #%ld: %ld\n", n, v);
}

size_t get_n_prime(size_t n) {
  if (n < 1)
    return 0; // Handle invalid input
  size_t count = 0;
  size_t candidate = 1;

  while (count < n) {
    candidate++;
    int is_prime = 1;

    for (size_t i = 2; i * i <= candidate; i++) {
      if (candidate % i == 0) {
        is_prime = 0;
        break;
      }
    }

    if (is_prime) {
      count++;
    }
  }

  return candidate;
}
