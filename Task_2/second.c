#include <sys/types.h>
#include <stdlib.h>
#include "sys/wait.h"
#include "stdio.h"
#include "unistd.h"

int main(int argc, char* argv[]) {
    int status;
    int pid = fork();
    if (pid == 0) {
        char *args[argc];
        for (int i = 1; i < argc; i++)
            args[i - 1] = argv[i];
        args[argc - 1] = NULL;
        execvp(args[0], args);
        exit(EXIT_SUCCESS);
    }
    waitpid(pid, &status, 0);
    if (status == 0) printf("Success!\n");
    else printf("Failed, exit code = %d\n", status);
    return 0;
}
