package igoroffline.bosonoga;

import igoroffline.bosonoga.generated.antlr.BosonogaBaseListener;
import igoroffline.bosonoga.generated.antlr.BosonogaParser;
import org.antlr.v4.runtime.ParserRuleContext;

public class BosonogaBaseListenerExtended extends BosonogaBaseListener {

  private BosonogaGlobal global;

  public BosonogaBaseListenerExtended(BosonogaGlobal global) {
    this.global = global;
  }

  public BosonogaGlobal getGlobal() {
    return global;
  }

  @Override
  public void enterBosonogamainentrypoint(BosonogaParser.BosonogamainentrypointContext ctx) {
    System.out.println("<enterBosonogamainentrypoint>");
  }

  @Override
  public void exitBosonogamainentrypoint(BosonogaParser.BosonogamainentrypointContext ctx) {
    System.out.println("<exitBosonogamainentrypoint>");
  }

  @Override
  public void enterBosonogamaincore(BosonogaParser.BosonogamaincoreContext ctx) {
    System.out.println("<enterBosonogamaincore>");
  }

  @Override
  public void exitBosonogamaincore(BosonogaParser.BosonogamaincoreContext ctx) {
    System.out.println("<exitBosonogamaincore>");
  }

  @Override
  public void enterBosonogaint32(BosonogaParser.Bosonogaint32Context ctx) {
    if (ctx.children.size() == 1) {
      final var first = ctx.children.getFirst();
      System.out.println("<enterBosonogaint32>");
      try {
        final var newValue = Integer.parseInt(first.getText());
        final var newSum = global.sum() + newValue;
        global = new BosonogaGlobal(global.name(), newSum);
      } catch (NumberFormatException ex) {
        System.err.println(first.getText() + " parsing-err-deae8523");
      }
    }
  }

  @Override
  public void exitBosonogaint32(BosonogaParser.Bosonogaint32Context ctx) {
    System.out.println("<exitBosonogaint32>");
  }

  @Override
  public void enterBosonogamainexitpoint(BosonogaParser.BosonogamainexitpointContext ctx) {
    System.out.println("<enterBosonogamainexitpoint>");
  }

  @Override
  public void exitBosonogamainexitpoint(BosonogaParser.BosonogamainexitpointContext ctx) {
    System.out.println("<exitBosonogamainexitpoint>");
  }

  @Override
  public void enterBosonogainit(BosonogaParser.BosonogainitContext ctx) {
    System.out.println("<enterBosonogainit>");
  }

  @Override
  public void exitBosonogainit(BosonogaParser.BosonogainitContext ctx) {
    System.out.println("<exitBosonogainit>");
  }

  @Override
  public void enterBosonogaimmutablevariable(BosonogaParser.BosonogaimmutablevariableContext ctx) {
    System.out.println("<enterBosonogaimmutablevariable>");
  }

  @Override
  public void exitBosonogaimmutablevariable(BosonogaParser.BosonogaimmutablevariableContext ctx) {
    System.out.println("<exitBosonogaimmutablevariable>");
  }

  @Override
  public void enterBosonogaset(BosonogaParser.BosonogasetContext ctx) {
    System.out.println("<enterBosonogaset>");
  }

  @Override
  public void exitBosonogaset(BosonogaParser.BosonogasetContext ctx) {
    System.out.println("<exitBosonogaset>");
  }

  @Override
  public void enterBosonogastring(BosonogaParser.BosonogastringContext ctx) {
    if (ctx.children.size() == 1) {
      final var first = ctx.children.getFirst();
      System.out.println("<enterBosonogastring>");
      try {
        final var name = first.getText();
        global = new BosonogaGlobal(name, global.sum());
      } catch (NumberFormatException ex) {
        System.err.println(first.getText() + " parsing-err-d9c33b6f");
      }
    }
  }

  @Override
  public void exitBosonogastring(BosonogaParser.BosonogastringContext ctx) {
    System.out.println("<exitBosonogastring>");
  }

  @Override
  public void enterEveryRule(ParserRuleContext ctx) {
    System.out.println("<enterEveryRule>");
  }

  @Override
  public void exitEveryRule(ParserRuleContext ctx) {
    System.out.println("<exitEveryRule>");
  }
}
