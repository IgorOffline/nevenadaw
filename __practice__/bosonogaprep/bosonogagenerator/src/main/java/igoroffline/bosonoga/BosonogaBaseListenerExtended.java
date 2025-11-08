package igoroffline.bosonoga;

import igoroffline.bosonoga.generated.antlr.BosonogaBaseListener;
import igoroffline.bosonoga.generated.antlr.BosonogaParser;
import org.antlr.v4.runtime.ParserRuleContext;

public class BosonogaBaseListenerExtended extends BosonogaBaseListener {

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
  public void enterEveryRule(ParserRuleContext ctx) {
    System.out.println("<enterEveryRule>");
  }

  @Override
  public void exitEveryRule(ParserRuleContext ctx) {
    System.out.println("<exitEveryRule>");
  }
}
