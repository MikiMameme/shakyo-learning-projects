//成績判定プログラム
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        while (true) {
            int score;

            //成績入力
            while (true) {
                System.out.print("点数を入力してください: ");
                score = scanner.nextInt();

                if (score < 0 || score > 100) {
                    System.out.println("エラー：不正な点数です");
                } else {
                    break;
                }
            }

            //成績判定
            if (score >= 90) {
                System.out.println("評価：A");
            } else if (score >= 60) {
                System.out.println("評価：B");
            } else {
                System.out.println("評価：C");
            }

            //続けるか確認する
            System.out.print("もう一度やりますか？(y/n): ");
            String answer = scanner.next();

            if (!answer.equals("y")) {
                break;
            }
        }

        System.out.println("プログラムを終了します。お疲れ様でした");
        scanner.close();
    }
}
