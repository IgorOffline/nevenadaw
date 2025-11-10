%{
#include "calc.tab.h"
%}

%%

[0-9]+      { yylval = atoi(yytext); return NUMBER; }
"ADO"       { return ADO; }
"quit"      { return QUIT; }
[ \t]       ; /* skip whitespace */
\n          { return '\n'; }
.           { printf("Unknown character: %s\n", yytext); }

%%

int yywrap() {
    return 1;
}