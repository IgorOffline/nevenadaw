package igoroffline;

import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.Session;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.testcontainers.cassandra.CassandraContainer;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;

@Testcontainers
public class BookIntegrationTest {

  @Container
  private static final CassandraContainer cassandra =
    new CassandraContainer("cassandra:latest");
  private static final String KEYSPACE = "test_keyspace";
  private static final String TABLE = "books";
  private static Session session;
  private static BookRepository bookRepository;

  @BeforeAll
  static void setup() {
    final var cluster = Cluster.builder()
      .addContactPoint(cassandra.getHost())
      .withPort(cassandra.getMappedPort(9042))
      .withoutJMXReporting()
      .build();

    session = cluster.connect();

    KeySpaceUtil.createKeyspace(KEYSPACE, "SimpleStrategy", 1, session);
    ColumnFamilyUtility.createColumnFamily(KEYSPACE, TABLE, session);

    bookRepository = new BookRepository(session, KEYSPACE, TABLE);
  }

  @AfterAll
  static void tearDown() {
    if (session != null) {
      session.close();
    }
  }

  @BeforeEach
  void clearData() {
    session.execute("TRUNCATE " + KEYSPACE + "." + TABLE);
  }

  @Test
  void findAllBooks() {
    session.execute("INSERT INTO " + KEYSPACE + "." + TABLE + " (id, title, subject) VALUES (uuid(), 'title-1', 'subject-1')");
    session.execute("INSERT INTO " + KEYSPACE + "." + TABLE + " (id, title, subject) VALUES (uuid(), 'title-2', 'subject-2')");
    session.execute("INSERT INTO " + KEYSPACE + "." + TABLE + " (id, title, subject) VALUES (uuid(), 'title-3', 'subject-3')");
    final var list = bookRepository.findAllBooks();

    Assertions.assertEquals(3, list.size());
  }
}