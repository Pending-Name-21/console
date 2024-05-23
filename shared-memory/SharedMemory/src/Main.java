import java.io.File;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;

public class Main {
    private static final String MMAP_PATH = "/home/fundacion/University/Fifth/SoftwareDevelopment/SharedMemory/text.txt";
    private static final int ROWS = 5;
    private static final int COLS = 5;

    static {
        System.load(new File("/home/fundacion/University/Fifth/SoftwareDevelopment/SharedMemory/libshlib.so")
                .getAbsolutePath());
    }

    public static void main(String[] args) throws Exception {
        try (RandomAccessFile sc = new RandomAccessFile(MMAP_PATH, "rw")) {
            MappedByteBuffer buffer = sc.getChannel().map(FileChannel.MapMode.READ_WRITE, 0, ROWS * COLS);

            char[][] charMatrix = {
                    { 'x', '.', '.', '.', '.' },
                    { '.', '.', '.', '.', '.' },
                    { '.', '.', '.', '.', '.' },
                    { '.', '.', '.', '.', '.' },
                    { '.', '.', '.', '.', '.' }
            };

            for (char[] row : charMatrix) {
                for (char c : row) {
                    buffer.put((byte) c);
                }
            }

            int x = 0;
            int y = 0;
            int rowToModify = 1;
            int colToModify = 0;
            char cursorChar = 'x';
            char pointChar = '.';
            int oldIndex, newIndex;

            oldIndex = x * COLS + y;
                buffer.put(oldIndex, (byte) pointChar);
                newIndex = rowToModify * COLS + colToModify;
                buffer.put(newIndex, (byte) cursorChar);
                y = rowToModify;
                rowToModify++;

/*             for (int i = 0; i < 5; i++) {
                oldIndex = x * COLS + y;
                buffer.put(oldIndex, (byte) pointChar);
                newIndex = rowToModify * COLS + colToModify;
                buffer.put(newIndex, (byte) cursorChar);
                y = rowToModify;
                rowToModify++;
                main.notifyMove(ROWS, COLS);
            } */

            /* rowToModify--;
            colToModify++;

            for (int i = 0; i < 5; i++) {
                oldIndex = x * COLS + y;
                buffer.put(oldIndex, (byte) pointChar);
                newIndex = rowToModify * COLS + colToModify;
                buffer.put(newIndex, (byte) cursorChar);
                x = colToModify;
                colToModify++;
                main.notifyMove(ROWS, COLS);
            } */

        }
    }

    // public native void notifyMove(int x, int y);
}
