package igoroffline.manymanyjava.manymanymicronaut;

import igoroffline.manymanyjava.manymanyscalacore.Sum;

public class Main {
    static void main(String[] args) {
        System.out.println("<START>");
        int v1 = Sum.var1();
        int v2 = Sum.var2();
        int s = Sum.sum();
        System.out.println("v1=" + v1 + ", v2=" + v2 + ", sum=" + s);
        System.out.println("<END>");
    }
}
