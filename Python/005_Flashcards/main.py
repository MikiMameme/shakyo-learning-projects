#単語帳アプリ

import random

#単語帳データ
words = {
    "apple": "りんご",
    "book": "本",
    "cat": "猫",
    "eat": "食べる",
    "fish": "魚",
    "go": "行く",
    "happy": "幸せな",
    "love": "愛",
    "run": "走る"
}

def quiz():
    english = random.choice(list(words.keys()))
    return english, words[english]

def main():
    print("=" * 30)
    print("英単語クイズ")
    print("=" * 30)

    score = 0
    total = 0

    while True:
        print("\n[Enter]で問題表示/[q]で終了")
        choice = input("> ").strip()
        if choice == "q":
            break

        #問題を出題
        english,japanese = quiz()
        total = total + 1

        print(f"\n問題{total}:{english}")
        answer = input("日本語の意味は？>").strip()

        if answer == japanese:
            print("正解")
            score += 1

        else:
            print(f"不正解です。正解は「{japanese}」です")

        print(f"現在のスコア：{score}/{total}")

    #最終結果を表示
    if total > 0:
        print("\n" + "=" * 30)
        print(" 最終結果")
        print("=" * 30)
        print(f"正解数：{score}/{total}")
        accuracy = (score / total) * 100
        print(f"正答率：{accuracy:.1f}%")

        if accuracy == 100:
            print("パーフェクトです")
        elif accuracy >= 80:
            print("よくできました")
        elif accuracy >= 60:
            print("いい感じです")
        else:
            print("もう少し頑張りましょう")

    print("\n終了します")

if __name__ == "__main__":
    main()
