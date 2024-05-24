import java.io.File;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;

public class Main {

    private static final String MMAP_PATH = "/home/fundacion/University/Fifth/SoftwareDevelopment/console/shared-memory/SharedMemory/text.txt";
    private static final int ROWS = 5;
    private static final int COLS = 5;

    static {
        System.load(new File("/home/fundacion/University/Fifth/SoftwareDevelopment/console/shared-memory/SharedMemory/libshlib.so")
                .getAbsolutePath());
    }

    public static void main(String[] args) throws Exception {
        Main mainTwo = new Main();

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
        }

        moveAndPrintMatrix(ROWS, COLS, mainTwo);
    }

    private static void moveAndPrintMatrix(int rows, int cols, Main mainTwo) throws Exception {
        try (RandomAccessFile sc = new RandomAccessFile(MMAP_PATH, "rw")) {
            MappedByteBuffer buffer = sc.getChannel().map(FileChannel.MapMode.READ_WRITE, 0, rows * cols);

            int x = 0, y = 0;

            int[][] movements = {
                    {0, 1},  // right
                    {1, 0},  // bottom
                    {0, -1}, // left
                    {-1, 0}  // top
            };

            for (int[] move : movements) {
                int newX = x + move[0];
                int newY = y + move[1];

                if (newX >= 0 && newX < rows && newY >= 0 && newY < cols) {
                    buffer.put(x * cols + y, (byte) '.');
                    buffer.put(newX * cols + newY, (byte) 'x');

                    x = newX;
                    y = newY;

                    mainTwo.notifyMove(rows, cols);
                    System.out.println();
                }
            }
        }
    }

    public native void notifyMove(int x, int y);
}
