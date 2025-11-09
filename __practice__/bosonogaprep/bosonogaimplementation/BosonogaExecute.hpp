#ifndef BOSONOGAIMPLEMENTATION_EXECUTE_H
#define BOSONOGAIMPLEMENTATION_EXECUTE_H

#include <filesystem>
#include <string>
#include <unordered_map>
#include "antlr4-runtime.h"
#include "BosonogaBaseListenerExtended.hpp"
#include "generated/BosonogaLexer.h"
#include "generated/BosonogaParser.h"
#include "BosonogaPrimitives.hpp"

class BosonogaExecute {
public:
    static bosonoga_string prepareTargetString() {
        std::filesystem::path cur = std::filesystem::current_path();
        while (true) {
            if (cur.filename() == BOSONOGA_IMPLEMENTATION_URI_STRING) {
                break;
            }
            auto parent = cur.parent_path();
            if (parent.empty() || parent == cur) { return BOSONOGA_EMPTY_STRING; }
            cur = parent;
        }
        const std::filesystem::path target = cur.parent_path() / BOSONOGA_MAIN_FILE_STRING;
        return target.string();
    }

    static int execute() {
        antlr4::ANTLRFileStream input;
        input.loadFromFile(prepareTargetString());
        igorofflinebosonogageneratedantlr::BosonogaLexer lexer(&input);
        antlr4::CommonTokenStream tokens(&lexer);
        igorofflinebosonogageneratedantlr::BosonogaParser parser(&tokens);
        BosonogaBaseListenerExtended listener(BosonogaGlobal(BOSONOGA_UNORDERED_MAP<BosonogaName, BosonogaSum>{}));
        antlr4::tree::ParseTreeWalker walker;
        auto tree = parser.bosonogamaincore();
        walker.walk(&listener, tree);
        const auto &global = listener.getGlobal();
        std::cout << global << std::endl;

        return BOSONOGA_ZERO;
    }
};

#endif //BOSONOGAIMPLEMENTATION_EXECUTE_H
