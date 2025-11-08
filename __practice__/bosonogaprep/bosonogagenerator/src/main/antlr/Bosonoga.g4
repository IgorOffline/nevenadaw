grammar Bosonoga;
bosonogamainentrypoint : 'BOSONOGA main' ;
bosonogaint32          : INT ;
bosonogamainexitpoint  : 'BOSONOGA fin' ;
bosonogamaincore       : bosonogamainentrypoint (bosonogaint32)* bosonogamainexitpoint ;
INT                    : [0-9]+ ;
WS                     : [ \t\r\n]+ -> skip ;