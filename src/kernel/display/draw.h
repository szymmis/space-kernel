#define VGA_BUFFER ((char *)0xA0000)
#define SCREEN_WIDTH 320
#define SCREEN_HEIGHT 200

void draw_rect(int x, int y, int w, int h);
void draw_img(char *img, int x, int y, int w, int h);
void draw_text(char *str, int x, int y);
void clear_screen();
