%{
#include "parser.tab.h"
%}

%%

[0-9]+  { yylval.number = atoi(yytext); return NUMBER; }
"ADA"   { return ADA; }
"EKI"   { return EKI; }
[ \t\n] ; /* skip whitespace */
.       { return yytext[0]; }

%%