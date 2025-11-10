#ifndef BOSONOGAIMPLEMENTATION_BOSONOGABASELISTENEREXTENDED_H
#define BOSONOGAIMPLEMENTATION_BOSONOGABASELISTENEREXTENDED_H

#include <iostream>
#include <utility>
#include "BosonogaGlobal.hpp"
#include "antlr4-runtime.h"
#include "generated/BosonogaBaseListener.h"
#include "generated/BosonogaParser.h"

class BosonogaBaseListenerExtended final : public igorofflinebosonogageneratedantlr::BosonogaBaseListener {
public:
  bool logLangTrace = false;

private:
  BosonogaGlobal global;
  BosonogaName currentImmutableVariableName = BosonogaName(BOSONOGA_EMPTY_STRING);

public:
  explicit BosonogaBaseListenerExtended(BosonogaGlobal global_)
    : global(std::move(global_)) {
  }

  [[nodiscard]] const BosonogaGlobal &getGlobal() const {
    return global;
  }

  void enterBosonogamainentrypoint(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamainentrypointContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogamainentrypoint" << std::endl;
    }
  }

  void exitBosonogamainentrypoint(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamainentrypointContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogamainentrypoint" << std::endl;
    }
  }

  void enterBosonogamainexitpoint(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamainexitpointContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogamainexitpoint" << std::endl;
    }
  }

  void exitBosonogamainexitpoint(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamainexitpointContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogamainexitpoint" << std::endl;
    }
  }

  void enterBosonogainit(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogainitContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogainit" << std::endl;
    }
  }

  void exitBosonogainit(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogainitContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogainit" << std::endl;
    }
  }

  void enterBosonogaimmutablevariable(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogaimmutablevariableContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogaimmutablevariable" << std::endl;
    }
  }

  void exitBosonogaimmutablevariable(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogaimmutablevariableContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogaimmutablevariable" << std::endl;
    }
  }

  void enterBosonogaset(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogasetContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogaset" << std::endl;
    }
  }

  void exitBosonogaset(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogasetContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogaset" << std::endl;
    }
  }

  void enterBosonogaint32(
    igorofflinebosonogageneratedantlr::BosonogaParser::Bosonogaint32Context *ctx) override {
    if (ctx && ctx->children.size() == BOSONOGA_ONE) {
      const auto first = ctx->children.front();
      if (logLangTrace) {
        std::cout << "<enterBosonogaint32>" << std::endl;
      }
      try {
        const int newValue = std::stoi(first->getText());
        if (const bosonoga_string key = currentImmutableVariableName.name; !key.empty()) {
          auto kv = global.nameSum;
          const auto it = kv.find(BosonogaName(key));
          bosonoga_int cur = BOSONOGA_ZERO;
          if (it != kv.end()) {
            cur = it->second.to_int();
            kv.erase(it);
          }
          const bosonoga_int newSumValue = cur + newValue;
          kv.emplace(BosonogaName(key), BosonogaSum(newSumValue));
          global.~BosonogaGlobal();
          new(&global) BosonogaGlobal(std::move(kv));
        }
      } catch (const std::exception &ex) {
        std::cerr << first->getText() << " parsing-err-e1efafdf" << std::endl;
      }
    }
  }

  void exitBosonogaint32(
    igorofflinebosonogageneratedantlr::BosonogaParser::Bosonogaint32Context *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogaint32" << std::endl;
    }
  }

  void enterBosonogastring(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogastringContext *ctx) override {
    if (ctx && ctx->children.size() == BOSONOGA_ONE) {
      const auto first = ctx->children.front();
      if (logLangTrace) {
        std::cout << "<enterBosonogastring>" << std::endl;
      }
      try {
        const bosonoga_string name = first->getText();
        if (!name.empty()) {
          auto kv = global.nameSum;
          if (!kv.contains(BosonogaName{name})) {
            kv.emplace(BosonogaName{name}, BosonogaSum(BOSONOGA_ZERO));
          }
          global.~BosonogaGlobal();
          new(&global) BosonogaGlobal(std::move(kv));
        }
        if (!name.empty() && currentImmutableVariableName.name != name) {
          currentImmutableVariableName.~BosonogaName();
          new(&currentImmutableVariableName) BosonogaName{name};
        }
      } catch (const std::exception &ex) {
        std::cerr << first->getText() << " parsing-err-b99c79ee" << std::endl;
      }
    }
  }

  void exitBosonogastring(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogastringContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogastring" << std::endl;
    }
  }

  void enterBosonogamaincore(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamaincoreContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterBosonogamaincore" << std::endl;
    }
  }

  void exitBosonogamaincore(
    igorofflinebosonogageneratedantlr::BosonogaParser::BosonogamaincoreContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitBosonogamaincore" << std::endl;
    }
  }

  void enterEveryRule(antlr4::ParserRuleContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "enterEveryRule" << std::endl;
    }
  }

  void exitEveryRule(antlr4::ParserRuleContext *ctx) override {
    (void) ctx;
    if (logLangTrace) {
      std::cout << "exitEveryRule" << std::endl;
    }
  }

  void visitTerminal(antlr4::tree::TerminalNode *node) override {
    (void) node;
    if (logLangTrace) {
      std::cout << "visitTerminal" << std::endl;
    }
  }

  void visitErrorNode(antlr4::tree::ErrorNode *node) override {
    (void) node;
    if (logLangTrace) {
      std::cout << "visitErrorNode" << std::endl;
    }
  }
};

#endif //BOSONOGAIMPLEMENTATION_BOSONOGABASELISTENEREXTENDED_H
