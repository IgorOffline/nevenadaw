#ifndef BOSONOGAIMPLEMENTATION_EXECUTE_H
#define BOSONOGAIMPLEMENTATION_EXECUTE_H

#include "antlr4-runtime.h"
#include "BosonogaBaseListenerExtended.hpp"
#include "generated/BosonogaLexer.h"
#include "generated/BosonogaParser.h"
#include <filesystem>
#include <string>

class Execute {
public:
    static std::string prepareTargetString() {
        std::filesystem::path cur = std::filesystem::current_path();
        while (true) {
            if (cur.filename() == "bosonogaimplementation") {
                break;
            }
            auto parent = cur.parent_path();
            if (parent.empty() || parent == cur) { return ""; }
            cur = parent;
        }
        std::filesystem::path target = cur.parent_path() / "main.bosonoga";
        return target.string();
    }

    static int execute() {
        antlr4::ANTLRFileStream input;
        input.loadFromFile(prepareTargetString());
        igorofflinebosonogageneratedantlr::BosonogaLexer lexer(&input);
        antlr4::CommonTokenStream tokens(&lexer);
        igorofflinebosonogageneratedantlr::BosonogaParser parser(&tokens);
        BosonogaBaseListenerExtended listener(BosonogaGlobal(BosonogaName(""), BosonogaSum(0)));
        antlr4::tree::ParseTreeWalker walker;
        auto tree = parser.bosonogamaincore();
        walker.walk(&listener, tree);
        const auto &global = listener.getGlobal();
        std::cout << global << std::endl;

        return 0;
    }
};

#endif //BOSONOGAIMPLEMENTATION_EXECUTE_H
