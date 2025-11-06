grammar BosonogaGrammar;
bosonogamainentrypoint : 'BOSONOGA main' ;
bosonogamaincore       : (bosonogamainentrypoint)* ;
WS                 : [ \t\r\n]+ -> skip ;
