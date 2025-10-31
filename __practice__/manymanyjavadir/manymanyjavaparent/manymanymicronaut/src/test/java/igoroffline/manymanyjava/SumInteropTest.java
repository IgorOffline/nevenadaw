package igoroffline.manymanyjava;

import igoroffline.MyRecord;
import igoroffline.manymanyjava.manymanyscalacore.Sum;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class SumInteropTest {

  @Test
  void sum_is_four() {
    int result = Sum.sum();
    assertThat(result).isEqualTo(4);
  }

  @Test
  void vars_are_two() {
    final var sumVar1 = new MyRecord(Sum.var1());
    final var sumVar2 = new MyRecord(Sum.var2());
    assertThat(sumVar1.myre()).isEqualTo(2);
    assertThat(sumVar2.myre()).isEqualTo(2);
  }
}
