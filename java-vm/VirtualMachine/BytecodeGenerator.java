package org.example;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;

public class BytecodeGenerator {
    public static void main(String[] args) {
        try {
            File compilationDirectory = new File("src/main/java/org/example");

            ProcessBuilder compileProcessBuilder = new ProcessBuilder("javac", "MiClase.java");
            compileProcessBuilder.directory(compilationDirectory);
            Process compileProcess = compileProcessBuilder.start();
            int compileExitCode = compileProcess.waitFor();

            if (compileExitCode == 0) {
                System.out.println("Compilación exitosa");

                File disassembleDirectory = new File("src/main/java/org/example");

                File outputFile = new File("src/main/java/org/example/bytecode.txt");

                ProcessBuilder disassembleProcessBuilder = new ProcessBuilder("javap", "-c", "MiClase.class");
                disassembleProcessBuilder.directory(disassembleDirectory);
                disassembleProcessBuilder.redirectOutput(outputFile);
                Process disassembleProcess = disassembleProcessBuilder.start();
                int disassembleExitCode = disassembleProcess.waitFor();

                if (disassembleExitCode == 0) {
                    System.out.println("Desensamblaje exitoso");
                } else {
                    System.out.println("Error durante el desensamblaje");
                }
            } else {
                System.out.println("Error durante la compilación");
            }
        } catch (IOException | InterruptedException e) {
            e.printStackTrace();
        }
    }
}
