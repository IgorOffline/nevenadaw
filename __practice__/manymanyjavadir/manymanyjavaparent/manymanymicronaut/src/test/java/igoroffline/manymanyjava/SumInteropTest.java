package igoroffline.manymanyjava;

import igoroffline.manymanyjava.manymanycassandra.bookexample.ExampleRecord;
import igoroffline.manymanyjava.manymanyscalacore.Sum;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class SumInteropTest {

  @Test
  void sum_is_four() {
    int result = Sum.sum();
    assertThat(result).isEqualTo(5);
  }

  @Test
  void vars_are_two() {
    final var sumVar1 = new ExampleRecord(Sum.var1());
    final var sumVar2 = new ExampleRecord(Sum.var2());
    assertThat(sumVar1.exampleInteger()).isEqualTo(2);
    assertThat(sumVar2.exampleInteger()).isEqualTo(3);
  }
}
