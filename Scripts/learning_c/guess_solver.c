#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>

#include "guessing.h"

const int RANGE = 1000;

void main()
{
    Game game;
    game_init(&game, RANGE);

    int left = 0;
    int right = RANGE;
    int guess = RANGE / 2;

    for (;;)
    {
        printf("Guess: %d\n", guess);
        int result = game_guess(&game, guess);

        if (result > 0)

            right = guess - 1;
        else if (result < 0)
            left = guess + 1;
        else
        {
            printf("\n");
            printf("Solved!\n");
            printf("Num: %d\n", guess);
            printf("Guesses: %d\n", game.guesses);
            break;
        }

        guess = (right - left) / 2 + left;
    }
}
