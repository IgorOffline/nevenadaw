package igoroffline;

import com.datastax.driver.core.Session;
import com.datastax.driver.core.querybuilder.QueryBuilder;

import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

public class BookRepository {
  private final Session session;
  private final String keyspace;
  private final String table;

  public BookRepository(Session session, String keyspace, String table) {
    this.session = session;
    this.keyspace = keyspace;
    this.table = table;
  }

  public List<Book> findAllBooks() {
    var query = QueryBuilder.select()
      .from(keyspace, table);

    var rows = session.execute(query);
    var books = new ArrayList<Book>();

    for (final var row : rows) {
      books.add(new Book(row.getUUID("id"), row.getString("title"), row.getString("subject")));
    }

    return books;
  }

  public Book findBookById(UUID id) {
    var query = QueryBuilder.select()
      .from(keyspace, table)
      .where(QueryBuilder.eq("id", id));

    var row = session.execute(query).one();

    if (row == null) {
      return null;
    }

    return new Book(
      row.getUUID("id"),
      row.getString("title"),
      row.getString("subject")
    );
  }

  public List<Book> findBooksByTitle(String title) {
    var query = QueryBuilder.select()
      .from(keyspace, table)
      .where(QueryBuilder.eq("title", title));

    var rows = session.execute(query);
    var books = new ArrayList<Book>();

    for (final var row : rows) {
      books.add(new Book(
        row.getUUID("id"),
        row.getString("title"),
        row.getString("subject")
      ));
    }

    return books;
  }

  public List<Book> findBooksBySubject(String subject) {
    var query = QueryBuilder.select()
      .from(keyspace, table)
      .where(QueryBuilder.eq("subject", subject));

    var rows = session.execute(query);
    var books = new ArrayList<Book>();

    for (final var row : rows) {
      books.add(new Book(
        row.getUUID("id"),
        row.getString("title"),
        row.getString("subject")
      ));
    }

    return books;
  }

  public void saveBook(Book book) {
    if (book == null) {
      throw new IllegalArgumentException("Book cannot be null");
    }

    var query = QueryBuilder.insertInto(keyspace, table)
      .value("id", book.getId() != null ? book.getId() : UUID.randomUUID())
      .value("title", book.getTitle())
      .value("subject", book.getSubject());

    session.execute(query);
  }

  public void saveBook(UUID id, String title, String subject) {
    if (title == null || title.trim().isEmpty()) {
      throw new IllegalArgumentException("Title cannot be null or empty");
    }

    var query = QueryBuilder.insertInto(keyspace, table)
      .value("id", id != null ? id : UUID.randomUUID())
      .value("title", title)
      .value("subject", subject);

    session.execute(query);
  }

  public boolean updateBook(Book book) {
    if (book == null || book.getId() == null) {
      throw new IllegalArgumentException("Book and book ID cannot be null");
    }

    var existingBook = findBookById(book.getId());
    if (existingBook == null) {
      return false;
    }

    var query = QueryBuilder.update(keyspace, table)
      .with(QueryBuilder.set("title", book.getTitle()))
      .and(QueryBuilder.set("subject", book.getSubject()))
      .where(QueryBuilder.eq("id", book.getId()));

    session.execute(query);
    return true;
  }

  public boolean updateBookTitle(UUID id, String newTitle) {
    if (id == null || newTitle == null || newTitle.trim().isEmpty()) {
      throw new IllegalArgumentException("ID and title cannot be null or empty");
    }

    var existingBook = findBookById(id);
    if (existingBook == null) {
      return false;
    }

    var query = QueryBuilder.update(keyspace, table)
      .with(QueryBuilder.set("title", newTitle))
      .where(QueryBuilder.eq("id", id));

    session.execute(query);
    return true;
  }

  public boolean deleteBook(UUID id) {
    if (id == null) {
      throw new IllegalArgumentException("Book ID cannot be null");
    }

    var existingBook = findBookById(id);
    if (existingBook == null) {
      return false;
    }

    var query = QueryBuilder.delete()
      .from(keyspace, table)
      .where(QueryBuilder.eq("id", id));

    session.execute(query);
    return true;
  }

  public long countAllBooks() {
    var query = QueryBuilder.select()
      .countAll()
      .from(keyspace, table);

    var row = session.execute(query).one();
    return row != null ? row.getLong(0) : 0;
  }

  public boolean existsById(UUID id) {
    if (id == null) {
      return false;
    }
    return findBookById(id) != null;
  }

  public void deleteAllBooks() {
    var query = QueryBuilder.truncate(keyspace, table);
    session.execute(query);
  }

  public List<Book> findAllBooksWithLimit(int limit) {
    if (limit <= 0) {
      throw new IllegalArgumentException("Limit must be positive");
    }

    var query = QueryBuilder.select()
      .from(keyspace, table)
      .limit(limit);

    var rows = session.execute(query);
    var books = new ArrayList<Book>();

    for (final var row : rows) {
      books.add(new Book(
        row.getUUID("id"),
        row.getString("title"),
        row.getString("subject")
      ));
    }

    return books;
  }
}