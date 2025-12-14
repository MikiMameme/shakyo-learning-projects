//シンプルなタイマー

import java.util.Scanner;

public class Main{
    public static void main(String[]args){
        Scanner scanner = new Scanner(System.in);

        System.out.println("===シンプルタイマー===");
        System.out.println("何秒後にアラームを鳴らしますか？");

        int seconds = scanner.nextInt();

        System.out.println("\n" + seconds + "秒のタイマーをセットしました\n");

        countdown(seconds);

        System.out.println("\n時間です");
        java.awt.Toolkit.getDefaultToolkit().beep();
        scanner.close();
    }

    static void countdown(int seconds){
        for(int i = seconds; i> 0; i--){
            int minutes = i / 60;
            int secs = i % 60;

            System.out.printf("\r残り時間；%02d:%02d", minutes, secs);

            try{
                Thread.sleep(1000);
            }catch(InterruptedException e){
                System.out.println("\nタイマーが中断されました");
                return;
            }
        }
        System.out.print("\r残り時間： 00:00");
    }
}