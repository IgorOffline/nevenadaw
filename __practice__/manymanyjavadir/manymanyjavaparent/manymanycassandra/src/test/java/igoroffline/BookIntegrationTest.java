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

import java.util.List;
import java.util.Objects;
import java.util.UUID;

@Testcontainers
class BookIntegrationTest {

  @Container
  private static final CassandraContainer CASSANDRA =
    new CassandraContainer("cassandra:5.0.6");

  private static final String KEYSPACE = "test_keyspace";
  private static final String TABLE = "books";
  private static final String REPLICATION_STRATEGY = "SimpleStrategy";
  private static final int REPLICATION_FACTOR = 1;
  private static final int EXPECTED_BOOK_COUNT = 3;
  private static final String TITLE_1 = "title-1";
  private static final String TITLE_2 = "title-2";
  private static final String TITLE_3 = "title-3";
  private static final String SUBJECT_1 = "subject-1";
  private static final String SUBJECT_2 = "subject-2";
  private static final String SUBJECT_3 = "subject-3";
  private static Cluster cluster;
  private static Session session;
  private static BookRepository bookRepository;

  @BeforeAll
  static void setup() {
    cluster = Cluster.builder()
      .addContactPoint(CASSANDRA.getHost())
      .withPort(CASSANDRA.getMappedPort(9042))
      .withoutJMXReporting()
      .build();

    session = cluster.connect();
    createTestSchema();
    bookRepository = new BookRepository(session, KEYSPACE, TABLE);
  }

  private static void createTestSchema() {
    KeySpaceUtil.createKeyspace(KEYSPACE, REPLICATION_STRATEGY, REPLICATION_FACTOR, session);
    ColumnFamilyUtility.createColumnFamily(KEYSPACE, TABLE, session);
  }

  @AfterAll
  static void tearDown() {
    closeResource(session, "session");
    closeResource(cluster, "cluster");
  }

  private static void closeResource(AutoCloseable resource, String resourceName) {
    if (resource != null) {
      try {
        resource.close();
      } catch (Exception e) {
        System.err.printf("Warning: Failed to close %s: %s%n", resourceName, e.getMessage());
      }
    }
  }

  @BeforeEach
  void clearData() {
    final String truncateQuery = "TRUNCATE " + KEYSPACE + "." + TABLE;
    session.execute(truncateQuery);
  }

  @Test
  void findAllBooks_WhenBooksExist_ReturnsAllBooks() {
    // Given
    insertTestBooks();

    // When
    List<Book> books = bookRepository.findAllBooks();

    // Then
    Assertions.assertEquals(EXPECTED_BOOK_COUNT, books.size());
    assertBooksContainExpectedData(books);
  }

  @Test
  void findAllBooks_WhenNoBooksExist_ReturnsEmptyList() {
    // When - no setup, table is empty due to @BeforeEach
    List<Book> books = bookRepository.findAllBooks();

    // Then
    Assertions.assertTrue(books.isEmpty(), "Expected empty list when no books exist");
  }

  @Test
  void findBookById_WhenBookExists_ReturnsBook() {
    // Given
    UUID bookId = UUID.randomUUID();
    insertBook(bookId, TITLE_1, SUBJECT_1);

    // When
    Book foundBook = bookRepository.findBookById(bookId);

    // Then
    Assertions.assertNotNull(foundBook, "Book should be found");
    Assertions.assertEquals(bookId, foundBook.getId());
    Assertions.assertEquals(TITLE_1, foundBook.getTitle());
    Assertions.assertEquals(SUBJECT_1, foundBook.getSubject());
  }

  @Test
  void findBookById_WhenBookDoesNotExist_ReturnsNull() {
    // Given
    UUID nonExistentId = UUID.randomUUID();

    // When
    Book foundBook = bookRepository.findBookById(nonExistentId);

    // Then
    Assertions.assertNull(foundBook, "Should return null for non-existent book");
  }

  @Test
  void saveBook_WhenValidBook_BookIsPersisted() {
    // Given
    Book newBook = new Book(UUID.randomUUID(), "New Book", "New Subject");

    // When
    bookRepository.saveBook(newBook);

    // Then
    Book savedBook = bookRepository.findBookById(newBook.getId());
    Assertions.assertNotNull(savedBook, "Book should be saved and retrievable");
    Assertions.assertEquals(newBook.getTitle(), savedBook.getTitle());
    Assertions.assertEquals(newBook.getSubject(), savedBook.getSubject());
  }

  @Test
  void deleteBook_WhenBookExists_BookIsRemoved() {
    // Given
    UUID bookId = UUID.randomUUID();
    insertBook(bookId, TITLE_1, SUBJECT_1);

    // Verify book exists
    Assertions.assertNotNull(bookRepository.findBookById(bookId), "Book should exist before deletion");

    // When
    bookRepository.deleteBook(bookId);

    // Then
    Book deletedBook = bookRepository.findBookById(bookId);
    Assertions.assertNull(deletedBook, "Book should be deleted");
  }

  private void insertTestBooks() {
    insertBook(UUID.randomUUID(), TITLE_1, SUBJECT_1);
    insertBook(UUID.randomUUID(), TITLE_2, SUBJECT_2);
    insertBook(UUID.randomUUID(), TITLE_3, SUBJECT_3);
  }

  private void insertBook(UUID id, String title, String subject) {
    final String insertQuery = "INSERT INTO " + KEYSPACE + "." + TABLE + " (id, title, subject) VALUES (?, ?, ?)";
    session.execute(insertQuery, id, title, subject);
  }

  private void assertBooksContainExpectedData(List<Book> books) {
    // Check that all expected titles are present
    List<String> expectedTitles = List.of(TITLE_1, TITLE_2, TITLE_3);
    List<String> actualTitles = books.stream()
      .map(Book::getTitle)
      .toList();

    Assertions.assertTrue(actualTitles.containsAll(expectedTitles),
      "All expected titles should be present in the result");

    // Verify no null books
    Assertions.assertTrue(books.stream().noneMatch(Objects::isNull),
      "No null books should be returned");

    // Verify all books have IDs
    Assertions.assertTrue(books.stream().allMatch(book -> book.getId() != null),
      "All books should have non-null IDs");
  }
}