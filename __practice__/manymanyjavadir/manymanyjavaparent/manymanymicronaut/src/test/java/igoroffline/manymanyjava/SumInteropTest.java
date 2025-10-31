package igoroffline.manymanyjava;

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
        assertThat(Sum.var1()).isEqualTo(2);
        assertThat(Sum.var2()).isEqualTo(2);
    }
}
