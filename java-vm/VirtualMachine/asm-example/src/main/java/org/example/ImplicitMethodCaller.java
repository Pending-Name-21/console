package org.example;

import org.objectweb.asm.*;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Paths;

public class ImplicitMethodCaller {

    public static void main(String[] args) throws Exception {
        String methodName = "main";

        byte[] modifiedBytes = transformBytecode("src/main/java/org/example/MiClase.class", methodName);

        CustomClassLoader loader = new CustomClassLoader();
        Class<?> modifiedClass = loader.defineClass("org.example.MiClase", modifiedBytes);

        Object instance = modifiedClass.getDeclaredConstructor(String.class)
                .newInstance("Nombre recibido");
        Method method = modifiedClass.getDeclaredMethod("getName");
        String result = (String) method.invoke(instance);
        System.out.println(result);
    }

    public static byte[] transformBytecode(String className, String methodName) throws Exception {
        byte[] bytecode = Files.readAllBytes(Paths.get("src/main/java/org/example/MiClase.class"));

        ClassReader reader = new ClassReader(bytecode);

        ClassWriter writer = new ClassWriter(reader, ClassWriter.COMPUTE_MAXS);

        ClassVisitor visitor = new ClassVisitor(Opcodes.ASM7, writer) {
            @Override
            public MethodVisitor visitMethod(int access, String name, String desc, String signature,
                    String[] exceptions) {
                MethodVisitor mv = super.visitMethod(access, name, desc, signature, exceptions);

                if (name.equals("implicitCall")) {
                    mv.visitVarInsn(Opcodes.ALOAD, 0);
                    mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "org.example.MiClase", methodName, "()V", false);
                }
                return mv;
            }
        };

        reader.accept(visitor, ClassReader.EXPAND_FRAMES);
        return writer.toByteArray();
    }

    static class CustomClassLoader extends ClassLoader {
        public Class<?> defineClass(String name, byte[] bytecode) {
            return defineClass(name, bytecode, 0, bytecode.length);
        }
    }
}
