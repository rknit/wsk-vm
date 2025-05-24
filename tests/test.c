#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

enum {
  ROCK = 0,
  PAPER = 1,
  SCISSORS = 2,
};

#include <unistd.h>

// [player][bot]
const int outcome[3][3] = {
    {0, -1, 1},
    {1, 0, -1},
    {-1, 1, 0},
};

int main() {
  srand(time(NULL));

  char buf[256];
  do {
    printf("\nrock / paper / scissors ?\n");
    printf(">> ");
    scanf(" %s", buf);
    if (strcmp(buf, "quit") == 0)
      break;

    int bot = rand() % 3;

    int player;
    if (strcmp(buf, "rock") == 0) {
      player = ROCK;
    } else if (strcmp(buf, "paper") == 0) {
      player = PAPER;
    } else if (strcmp(buf, "scissors") == 0) {
      player = SCISSORS;
    } else {
      printf("invalid choice!\n");
      continue;
    }

    int r = outcome[player][bot];
    if (r == 0) {
      printf("\nDRAW!\n");
    } else if (r == 1) {
      printf("\nWIN!\n");
    } else if (r == -1) {
      printf("\nLOSE!\n");
    }

  } while (1);
}
