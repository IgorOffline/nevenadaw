grammar Bosonoga;
bosonogamainentrypoint : 'BOSONOGA main' ;
bosonogamaincore       : (bosonogamainentrypoint)* ;
WS                     : [ \t\r\n]+ -> skip ;