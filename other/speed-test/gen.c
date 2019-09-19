#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char*argv[]) {
    if (argc < 3) {
        printf("3 args required - supplied %d\n", argc);
        return -1;
    }

    FILE * fh_domains = fopen(argv[1], "r");
    FILE * fh_out = fopen(argv[3], "w");

    char * domain= NULL;
    char * word= NULL;
    size_t leni = 0;
    size_t lenj = 0;
    ssize_t i;
    ssize_t j;

    
    while((i = getline(&domain, &leni, fh_domains)) != -1) {
        domain[strcspn(domain, "\n")] = 0;
        FILE * fh_wordlist = fopen(argv[2], "r");
        while((j = getline(&word, &lenj, fh_wordlist)) != -1) {
            word[strcspn(word, "\n")] = 0;
            fprintf(fh_out, "%s.%s\n", word, domain);
        }
        fclose(fh_wordlist);
    }

    fclose(fh_out);
    fclose(fh_domains);

    return 1;
}
