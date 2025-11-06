package igoroffline.zigjayserver;

import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public record Payload(int id, String name) {
}
