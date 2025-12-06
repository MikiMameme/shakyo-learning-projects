//じゃんけんゲーム

fun main(){
    println("===じゃんけんゲーム===")
    println("0:グー,1:チョキ,2:パー")
    print("あなたの手を選んでね :")

    val playerHand = readln().toInt()
    val computerHand = (0..2).random()

    val hands = listOf("グー","チョキ","パー")

    println("あなた：${hands[playerHand]}")
    println("コンピューター：${hands[computerHand]}")

    val result = when {
        playerHand == computerHand -> "引き分け"
        (playerHand == 0 && computerHand == 1) ||
                (playerHand == 1 && computerHand == 2) ||
                (playerHand == 2 && computerHand ==0) -> "あなたの勝ち"
        else -> "コンピューターの勝ち"
    }

    println(result)
}