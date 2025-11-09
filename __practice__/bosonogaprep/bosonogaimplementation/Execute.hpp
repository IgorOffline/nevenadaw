#ifndef BOSONOGAIMPLEMENTATION_EXECUTE_H
#define BOSONOGAIMPLEMENTATION_EXECUTE_H

#include "antlr4-runtime.h"
#include "BosonogaBaseListenerExtended.hpp"
#include "generated/BosonogaLexer.h"
#include "generated/BosonogaParser.h"

class Execute {
public:
    static int execute() {
        antlr4::ANTLRFileStream input;
        input.loadFromFile("..\\..\\main.bosonoga");
        igorofflinebosonogageneratedantlr::BosonogaLexer lexer(&input);
        antlr4::CommonTokenStream tokens(&lexer);
        igorofflinebosonogageneratedantlr::BosonogaParser parser(&tokens);
        BosonogaBaseListenerExtended listener;
        antlr4::tree::ParseTreeWalker walker;
        walker.walk(&listener, parser.bosonogamaincore());

        return 0;
    }
};

#endif //BOSONOGAIMPLEMENTATION_EXECUTE_H
