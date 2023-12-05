import java.util.Enumeration;
import java.util.Hashtable;
import java.util.Scanner;
import java.io.File;
import java.util.Dictionary;

public class D1 {
    public static void main(String[] args) {

        Dictionary<String, Integer> dict = new Hashtable<>();
        dict.put("one", 1);
        dict.put("two", 2);
        dict.put("three", 3);
        dict.put("four", 4);
        dict.put("five", 5);
        dict.put("six", 6);
        dict.put("seven", 7);
        dict.put("eight", 8);
        dict.put("nine", 9);

        int sum = 0;
        Scanner s = new Scanner(System.in);
        while (true) {
            String line = s.nextLine();
            if (line.isEmpty() || line.isBlank()) break;
            int firstNum = 0, lastNum = 0, firstIndex = 0, lastIndex = 0;
            Boolean found = false;
            int i = 0;
            for (char ch: line.toCharArray()) {
                if (Character.isDigit(ch)) {
                    if (!found) {firstNum = ch - '0';firstIndex = i;}
                    lastNum = ch - '0'; lastIndex = i;
                    found = true;
                } i++;
            }

            Enumeration<String> k = dict.keys();
            while (k.hasMoreElements()) {
                String key = k.nextElement();
                System.out.printf("%d %s\n", dict.get(key), key);
                int index = line.indexOf(key);
                System.out.println(dict.get(key) + " at index " + index);
                if (index != -1) {
                    if (index < firstIndex || !found) {
                        firstNum = dict.get(key);
                        firstIndex = index;
                        found = true;
                    }
                    if (index > lastIndex || !found) {
                        lastNum = dict.get(key);
                        lastIndex = index;
                        found = true;
                    }
                }
            }

            int completeNum = firstNum * 10 + lastNum;
            sum += completeNum;
            System.out.println(completeNum);
        }
        System.out.printf("\n%d", sum);
    }
}
