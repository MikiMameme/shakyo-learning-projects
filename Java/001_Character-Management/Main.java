public class Main {
    public static void main(String[]args){
        System.out.println("===RPGバトル===");

        Character hero = new Character("勇者",100,25);
        Character monster = new Character("スライム",50,15);

        hero.showStatus();
        monster.showStatus();

        System.out.println("---バトル開始---\n");

        int turn = 1;
        while(hero.isAlive() && monster.isAlive()){
            System.out.println("【ターン" + turn + "】");

            //攻撃(勇者がモンスターへ)
            hero.attackTo(monster);
            System.out.println();

            //攻撃(モンスターが勇者へ、モンスターが生きていたら)
            if(monster.isAlive()){
                monster.attackTo(hero);
                System.out.println();
            }
            turn++;
        }

        //結果
        System.out.println("---バトル終了---");
        if(hero.isAlive()){
            System.out.println(hero.getName() + "の勝利");
        }else{
            System.out.println(monster.getName() + "の勝利…");
        }
    }
}
