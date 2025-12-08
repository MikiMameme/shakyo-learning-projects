public class Character {
    private String name;
    private int hp;
    private int attack;

    //コンストラクタ
    public Character(String name,int hp,int attack){
        this.name = name;
        this.hp = hp;
        this.attack = attack;
    }

    //攻撃
    public void attackTo(Character target){
        System.out.println(this.name+"の攻撃！");
        target.damage(this.attack);
    }

    //ダメージを受ける
    public void damage(int damage){
        this.hp -= damage;
        System.out.println(this.name + "は" + damage + "のダメージを受けた！");

        if(this.hp <= 0){
            this.hp = 0;
            System.out.println(this.name + "は倒れた…");
        }else{
            System.out.println(this.name + "の残りHP：" + this.hp);
        }
    }

    //生存確認
    public boolean isAlive(){
     return this.hp > 0;
    }

    //ステータス
    public void showStatus(){
        System.out.println("【" + this.name + "】");
        System.out.println("HP：" + this.hp);
        System.out.println("攻撃力：" + this.attack);
        System.out.println();
    }

    //ゲッター
    public String getName(){
        return this.name;
    }
}
