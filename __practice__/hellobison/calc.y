%{
#include <stdio.h>
#include <stdlib.h>

int yylex(void);
void yyerror(const char *s);

int main() {
    printf("Calculator ready! Enter expressions like '2 ADO 3' or 'quit' to exit.\n");
    yyparse();
    return 0;
}
%}

%token NUMBER
%token ADO
%token QUIT

%%

input:    /* empty */
        | input line
        ;

line:     expression '\n'   { printf("Result: %d\n", $1); }
        | QUIT '\n'         { printf("Goodbye!\n"); exit(0); }
        | '\n'              /* empty line */
        ;

expression: NUMBER ADO NUMBER   { $$ = $1 + $3; printf("Transformed %d ADO %d into ", $1, $3); }
          | NUMBER              { $$ = $1; }
          ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}