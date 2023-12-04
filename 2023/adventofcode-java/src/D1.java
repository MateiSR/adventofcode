import java.util.Scanner;
import java.io.File;
public class D1 {
    public static void main(String[] args) {
        int sum = 0;
        Scanner s = new Scanner(System.in);
        while (true) {
            String line = s.nextLine();
            if (line.isEmpty() || line.isBlank()) break;
            int firstNum = -1, lastNum = 0;
            for (char ch: line.toCharArray()) {
                if (Character.isDigit(ch)) {
                    if (firstNum == -1) firstNum = ch - '0';
                    lastNum = ch - '0';
                }
            }
            int completeNum = firstNum * 10 + lastNum;
            sum += completeNum;
            System.out.println(completeNum);
        }
        System.out.printf("\n%d", sum);
    }
}
