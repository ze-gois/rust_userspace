#include <unistd.h>

int main(void) {
    static const char message[] = "dynamic fixture\n";
    write(1, message, sizeof(message) - 1);
    return 42;
}
