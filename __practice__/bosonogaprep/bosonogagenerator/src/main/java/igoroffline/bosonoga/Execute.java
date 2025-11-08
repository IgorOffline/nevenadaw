package igoroffline.bosonoga;

import igoroffline.bosonoga.generated.antlr.BosonogaLexer;
import igoroffline.bosonoga.generated.antlr.BosonogaParser;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Optional;

public class Execute {

  public void execute() {
    final var lexerRaw = prepareLexer();
    if (lexerRaw.isEmpty()) {
      return;
    }
    final var lexer = lexerRaw.get();
    final var listener = new BosonogaBaseListenerExtended();
    final var tokens = new CommonTokenStream(lexer);
    final var parser = new BosonogaParser(tokens);
    final var tree = parser.bosonogamaincore();
    final var walker = new ParseTreeWalker();
    walker.walk(listener, tree);
  }

  public Optional<BosonogaLexer> prepareLexer() {
    try {
      return Optional.of(new BosonogaLexer(CharStreams.fromFileName("main.bosonoga", StandardCharsets.UTF_8)));
    } catch (IOException ex) {
      System.err.println(ex.getMessage());
    }

    return Optional.empty();
  }
}
