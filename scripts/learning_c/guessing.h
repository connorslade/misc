typedef struct Game
{
    int actual;
    int guesses;
} Game;

void game_init(Game *game, int max);
int game_guess(Game *game, int guess);