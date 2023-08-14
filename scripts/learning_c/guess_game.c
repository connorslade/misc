#include "guessing.h"

void main()
{
    Game game;
    game_init(&game, 1000);

    for (;;)
    {
        int guess;
        printf("> ");
        scanf("%d", &guess);

        int result = game_guess(&game, guess);
        if (result > 0)
            printf("Too High\n");
        else if (result < 0)
            printf("Too Low\n");
        else
        {
            printf("Slayyy\n");
            printf("Guesses: %d", game.guesses);
            break;
        }
    }
}