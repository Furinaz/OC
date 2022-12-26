#include "stdio.h"
#include "unistd.h"

int main() {
    int waitTime = 20;

    if (fork() == 0) {
        sleep(waitTime);
        return 0;
    }
    if (fork() == 0) {
        sleep(waitTime);
        return 0;
    }
    if (fork() == 0) {
        sleep(waitTime);
        return 0;
    }
    if (fork() == 0) {
        if (fork() == 0){
            sleep(waitTime);
            return 0;
        }
    } else {
        if (fork() == 0) {
            sleep(waitTime);
            return 0;
        }
    }

    sleep(waitTime);
    return 0;
}