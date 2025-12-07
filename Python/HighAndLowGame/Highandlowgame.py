#ハイ＆ロー(数当てゲーム)

import random

def main():
    print("===数当てゲーム===")
    print("1から100までの数字を入れてください")

    answer = random.randint(1,100)
    attempts = 0

    while True:
        try:
            guess = int(input("予想 :"))
            attempts += 1

            if guess < answer:
                print("もっと大きいです🔼")

            elif guess > answer:
                print("もっと小さいです🔽")

            else:
                print(f"\n正解です。答えは{answer}でした")
                print(f"チャレンジ回数: {attempts}回")

                if attempts <= 5:
                    print("すごいですね！")
                elif attempts <= 10:
                    print("いい感じですね")
                else:
                    print("よくできました")
                break

        except ValueError:
            print("数字を入力してください")

if __name__ == "__main__":
    main()