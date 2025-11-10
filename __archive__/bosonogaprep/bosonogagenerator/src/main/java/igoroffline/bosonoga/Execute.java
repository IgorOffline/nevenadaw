package igoroffline.bosonoga;

import igoroffline.bosonoga.generated.antlr.BosonogaLexer;
import igoroffline.bosonoga.generated.antlr.BosonogaParser;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;

public class Execute {

  public void execute() {
    final var lexerRaw = prepareLexer();
    if (lexerRaw.isEmpty()) {
      return;
    }
    final var lexer = lexerRaw.get();
    final var listener = new BosonogaBaseListenerExtended(new BosonogaGlobal(new BosonogaName(""), new BosonogaSum(0)));
    final var tokens = new CommonTokenStream(lexer);
    final var parser = new BosonogaParser(tokens);
    final var tree = parser.bosonogamaincore();
    final var walker = new ParseTreeWalker();
    walker.walk(listener, tree);

    final var global = listener.getGlobal();
    System.out.println(global);
  }

  private Optional<BosonogaLexer> prepareLexer() {
    final String target = prepareTargetString();
    if (target.isBlank() || !Files.exists(Path.of(target))) {
      System.err.println("err-uri-b900b815");
      return Optional.empty();
    }
    try {
      return Optional.of(new BosonogaLexer(CharStreams.fromFileName(target, StandardCharsets.UTF_8)));
    } catch (IOException ex) {
      System.err.println(ex.getMessage());
    }

    return Optional.empty();
  }

  private String prepareTargetString() {
    Path cur = Path.of("").toAbsolutePath();
    while (true) {
      Path parent = cur.getParent();
      if (parent == null || parent.equals(cur)) {
        break;
      }
      Path implementationDir = parent.resolve("bosonogaimplementation");
      if (Files.isDirectory(implementationDir)) {
        Path target = parent.resolve("main.bosonoga");
        return target.toString();
      }
      cur = parent;
    }

    return "";
  }
}
