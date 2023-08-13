#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "guessing.h"

void game_init(Game *game, int max)
{
    srand(time(0));
    game->actual = rand() % max + 1;
    game->guesses = 0;
}

int game_guess(Game *game, int guess)
{
    game->guesses++;
    if (guess > game->actual)
        return 1;
    else if (guess < game->actual)
        return -1;
    else
        return 0;
}
