package igoroffline.zigjayserver;

import io.micronaut.http.MediaType;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import org.slf4j.Logger;

import static org.slf4j.LoggerFactory.getLogger;

@Controller
public class PayloadController {

  private static final Logger log = getLogger(PayloadController.class);

  @Get(produces = MediaType.APPLICATION_JSON)
  public Payload getPayload() {
    log.info("GET payload");
    return new Payload(1, "My나는ŠĐŽČĆPayload");
  }
}
