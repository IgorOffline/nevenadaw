%{
#include <stdio.h>
#include <stdlib.h>

int yyparse(void);
void yyerror(const char *s);
int yylex(void);

int main() {
    yyparse();
    return 0;
}
%}

%union {
    int number;
}

%token <number> NUMBER ADA EKI

%type <number> expression input

%%

input:
    /* empty */
    | expression EKI NUMBER {
        if ($1 == $3) {
            printf("Correct! %d equals %d\n", $1, $3);
        } else {
            printf("Error: %d does not equal %d\n", $1, $3);
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