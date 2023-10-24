#define VGA_BUFFER ((char *)0xA0000)

void draw_rect(int x, int y, int w, int h);
void draw_img(char *img, int x, int y, int w, int h);
