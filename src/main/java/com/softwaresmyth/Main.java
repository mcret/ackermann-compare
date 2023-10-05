package com.softwaresmyth;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Duration;
import java.time.Instant;

public class Main {

    public static void main(String[] args) {
		int x = 4;
		int y = 1;
		long expected = 65533;

		System.out.println("Ackermann(" + x + "," + y + ")");
		System.out.println();

		//do it with the library
		long result = -1;
        Path path = Paths.get("/home/numbers/code/ackermann/target/release/libackermann.so");
        System.load(path.toAbsolutePath().toString());
        Instant before = Instant.now();
        try {
			result = Main.ackermann(x, y);
		}
        catch (Exception e) {
            System.out.println(e.getMessage());
        }
        Instant after = Instant.now();
        Duration duration = Duration.between(before, after);

        System.out.println("With library:");
		System.out.printf("Result:%d\n", result);
        System.out.println(duration.toString());
        System.out.println();

        //do it with java
		result = -1;
		System.out.println("With java:");
		Instant beforeJava = Instant.now();
		Instant afterJava;
        try {
			result = ackermann_java(x, y);
			afterJava = Instant.now();
			assert result == expected;
		}
        catch (StackOverflowError e) {
            afterJava = Instant.now();
            System.out.println("Stack overflow.");
        }
        Duration durationJava = Duration.between(beforeJava, afterJava);
		System.out.printf("Result:%d\n", result);
        System.out.println(durationJava.toString());
    }

    public static long ackermann_java(long x, long y) {
        if (0 == x) {
            return y + 1;
        }
		if (0 == y) {
            return ackermann_java(x - 1, 1);
        }
		return ackermann_java(x - 1, ackermann_java(x, y - 1));
    }

    private static native long ackermann(long m, long n);
}