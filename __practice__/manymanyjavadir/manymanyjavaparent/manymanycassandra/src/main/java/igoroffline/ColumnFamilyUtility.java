package igoroffline;

import com.datastax.driver.core.Session;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ColumnFamilyUtility {

  private static final Logger log = LoggerFactory.getLogger(ColumnFamilyUtility.class);

  public static void createColumnFamily(String keyspaceName, String tableName, Session session) {

    final var query = "CREATE TABLE IF NOT EXISTS " +
      keyspaceName + "." + tableName +
      "(" +
      "id uuid PRIMARY KEY, " +
      "title text," +
      "subject text);";
    log.info(query);
    final var rs = session.execute(query);
    log.info("{}", rs.getExecutionInfo().getTriedHosts());
  }
}