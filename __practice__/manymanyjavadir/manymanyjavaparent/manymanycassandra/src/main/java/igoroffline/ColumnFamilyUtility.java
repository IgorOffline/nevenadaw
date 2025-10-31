package igoroffline;

import com.datastax.driver.core.Session;

public class ColumnFamilyUtility {

  public static void createColumnFamily(String keyspaceName, String tableName, Session session) {

    final var query = "CREATE TABLE IF NOT EXISTS " +
      keyspaceName + "." + tableName +
      "(" +
      "id uuid PRIMARY KEY, " +
      "title text," +
      "subject text);";
    System.out.println(query);
    final var rs = session.execute(query);
    System.out.println(rs.getExecutionInfo().getTriedHosts());
  }
}