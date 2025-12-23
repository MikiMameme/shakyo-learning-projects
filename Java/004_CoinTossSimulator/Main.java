//コイントスシミュレーター

import java.util.ArrayList;
import java.util.Random;
import java.util.Scanner;

public class Main{
    private static final Random random = new Random();
    private static final ArrayList<String> history = new ArrayList<>();

    public static String flip(){
        return random.nextBoolean() ? "表" : "裏";
    }

    //統計情報
    public static void showStatus(){
        if(history.isEmpty()){
            System.out.println("まだコインを投げていません");
            return;
        }

        int heads = 0;
        int tails = 0;

        for (String result : history){
            if(result.equals("表")){
                heads++;
            }else{
                tails++;
        }
    }
        int total = history.size();
        double headsPercent = (heads * 100.0) / total;
        double tailsPercent = (tails * 100.0) / total;

        System.out.println("\n===統計情報===");
        System.out.println("合計回数：" + total + "回");
        System.out.printf("表： %d回 (%.1f%%)%n", heads, headsPercent);
        System.out.printf("裏： %d回 (%.1f%%)%n", tails, tailsPercent);
}

//履歴
public static void showHistory(){
    if(history.isEmpty()){
        System.out.println("まだコインを投げていません");
        return;
    }

    System.out.println("\n===履歴===");
    for (int i = 0; i < history.size(); i++){
        System.out.println(history.get(i));
        if ((i + 1) % 10 == 0){
            System.out.println();
        } else {
            System.out.print(" ");
        }
    }
    System.out.println();
}

public static void main(String[] args){
        Scanner scanner =new Scanner(System.in);

        System.out.println("=====コイントスシミュレーター=====");

        while(true){
            System.out.println("\n[Enter] コインを投げる");
            System.out.println("[s] 統計表示");
            System.out.println("[h] 履歴表示");
            System.out.println("[q] 終了");
            System.out.print(">");

            String input = scanner.nextLine().trim().toLowerCase();

            if(input.equals("q")){
                break;
            } else if(input.equals("s")){
                showStatus();
            } else if(input.equals("h")){
                showHistory();
            } else {
                String result = flip();
                history.add(result);

                System.out.println("\n結果：" + result);
                System.out.println("（合計 " + history.size() + "回）");
            }
        }

        if(!history.isEmpty()){
            System.out.println("-----最終結果-----");
            showStatus();
        }

        System.out.println("\n終了します");
        scanner.close();
    }
}