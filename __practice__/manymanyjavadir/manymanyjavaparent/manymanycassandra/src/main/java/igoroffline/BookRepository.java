package igoroffline;

import com.datastax.driver.core.Session;
import com.datastax.driver.core.querybuilder.QueryBuilder;

import java.util.ArrayList;
import java.util.List;

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
}