package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

public class Main {

    public static void main(String[] args) {
        try {
            byte[] bytecode = Files.readAllBytes(Paths.get("src/main/java/org/example/Clase.class"));

            ClassReader reader = new ClassReader(bytecode);

            ClassVisitor visitor = new ClassVisitor(Opcodes.ASM8) {
                @Override
                public MethodVisitor visitMethod(int access, String name, String descriptor, String signature,
                                                 String[] exceptions) {
                    System.out.println("Method: " + name);

                    MethodVisitor methodVisitor = new MethodVisitor(Opcodes.ASM8) {
                        @Override
                        public void visitCode() {
                            System.out.println("  Code:");
                        }

                        @Override
                        public void visitInsn(int opcode) {
                            System.out.println("    Instruction: " + opcode);
                        }
                    };

                    return methodVisitor;
                }
            };

            reader.accept(visitor, 0);

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
