package igoroffline.manymanyjava.manymanycassandra.bookexample;

import com.datastax.driver.core.Session;

public class KeySpaceUtil {

  public static void createKeyspace(String keyspaceName,
                                    String replicationStrategy,
                                    int replicationFactor,
                                    Session session) {
    final var query = "CREATE KEYSPACE IF NOT EXISTS " +
      keyspaceName +
      " WITH replication = {" +
      "'class':'" +
      replicationStrategy +
      "','replication_factor':" +
      replicationFactor +
      " };";

    System.out.println(query);
    session.execute(query);
  }
}