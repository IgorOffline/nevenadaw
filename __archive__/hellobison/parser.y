%{
#include <stdio.h>
#include <stdlib.h>

extern FILE *yyin;

int yyparse(void);
void yyerror(const char *s);
int yylex(void);

int main() {
    const char *filename = "adaeki.txt";
    yyin = fopen(filename, "r");
    if (yyin == NULL) {
        fprintf(stderr, "Error: Could not open file '%s'\n", filename);
        return EXIT_FAILURE;
    }
    printf("Reading input from file: %s\n", filename);
    yyparse();
    fclose(yyin);
    return EXIT_SUCCESS;
}
%}

%union {
    int number;
}

%token <number> NUMBER ADA EKI

%type <number> expression input

%%

input:
    expression EKI NUMBER {
        if ($1 == $3) {
            printf("YES\n", $1, $3);
        } else {
            printf("NO\n", $1, $3);
        }
    }
    ;

expression:
    NUMBER ADA NUMBER { $$ = $1 + $3; }
    ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}