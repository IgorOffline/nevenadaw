package igoroffline.bosonoga;

import igoroffline.bosonoga.generated.antlr.BosonogaBaseListener;
import igoroffline.bosonoga.generated.antlr.BosonogaParser;
import org.antlr.v4.runtime.ParserRuleContext;

public class BosonogaBaseListenerExtended extends BosonogaBaseListener {

  private final boolean logLangTrace = false;
  private BosonogaGlobal global;

  public BosonogaBaseListenerExtended(BosonogaGlobal global) {
    this.global = global;
  }

  public BosonogaGlobal getGlobal() {
    return global;
  }

  @Override
  public void enterBosonogamainentrypoint(BosonogaParser.BosonogamainentrypointContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogamainentrypoint>");
    }
  }

  @Override
  public void exitBosonogamainentrypoint(BosonogaParser.BosonogamainentrypointContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogamainentrypoint>");
    }
  }

  @Override
  public void enterBosonogamaincore(BosonogaParser.BosonogamaincoreContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogamaincore>");
    }
  }

  @Override
  public void exitBosonogamaincore(BosonogaParser.BosonogamaincoreContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogamaincore>");
    }
  }

  @Override
  public void enterBosonogaint32(BosonogaParser.Bosonogaint32Context ctx) {
    if (ctx.children.size() == 1) {
      final var first = ctx.children.getFirst();
      if (logLangTrace) {
        System.out.println("<enterBosonogaint32>");
      }
      try {
        final var newValue = Integer.parseInt(first.getText());
        final var newSum = global.sum().sum() + newValue;
        global = new BosonogaGlobal(global.name(), new BosonogaSum(newSum));
      } catch (NumberFormatException ex) {
        System.err.println(first.getText() + " parsing-err-deae8523");
      }
    }
  }

  @Override
  public void exitBosonogaint32(BosonogaParser.Bosonogaint32Context ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogaint32>");
    }
  }

  @Override
  public void enterBosonogamainexitpoint(BosonogaParser.BosonogamainexitpointContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogamainexitpoint>");
    }
  }

  @Override
  public void exitBosonogamainexitpoint(BosonogaParser.BosonogamainexitpointContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogamainexitpoint>");
    }
  }

  @Override
  public void enterBosonogainit(BosonogaParser.BosonogainitContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogainit>");
    }
  }

  @Override
  public void exitBosonogainit(BosonogaParser.BosonogainitContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogainit>");
    }
  }

  @Override
  public void enterBosonogaimmutablevariable(BosonogaParser.BosonogaimmutablevariableContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogaimmutablevariable>");
    }
  }

  @Override
  public void exitBosonogaimmutablevariable(BosonogaParser.BosonogaimmutablevariableContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogaimmutablevariable>");
    }
  }

  @Override
  public void enterBosonogaset(BosonogaParser.BosonogasetContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterBosonogaset>");
    }
  }

  @Override
  public void exitBosonogaset(BosonogaParser.BosonogasetContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogaset>");
    }
  }

  @Override
  public void enterBosonogastring(BosonogaParser.BosonogastringContext ctx) {
    if (ctx.children.size() == 1) {
      final var first = ctx.children.getFirst();
      if (logLangTrace) {
        System.out.println("<enterBosonogastring>");
      }
      try {
        final var name = first.getText();
        global = new BosonogaGlobal(new BosonogaName(name), global.sum());
      } catch (NumberFormatException ex) {
        System.err.println(first.getText() + " parsing-err-d9c33b6f");
      }
    }
  }

  @Override
  public void exitBosonogastring(BosonogaParser.BosonogastringContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitBosonogastring>");
    }
  }

  @Override
  public void enterEveryRule(ParserRuleContext ctx) {
    if (logLangTrace) {
      System.out.println("<enterEveryRule>");
    }
  }

  @Override
  public void exitEveryRule(ParserRuleContext ctx) {
    if (logLangTrace) {
      System.out.println("<exitEveryRule>");
    }
  }
}
