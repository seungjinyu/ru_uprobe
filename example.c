#include <stdio.h>
#include <unistd.h>

int add_rust_aya(int a, int b) {
    return a + b;
}

int main() {

    pid_t pid;
    pid = getpid();

    int result = add_rust_aya(3, 4);
    printf("Process ID: %d\n", pid);
    printf("Result: %d\n", result);
    return 0;
}