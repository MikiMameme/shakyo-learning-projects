import random

def omikuji():
    fortunes = ["大吉","中吉","小吉","吉","末吉","凶","大凶"]
    messages = {
        "大吉": "何事もうまくいくでしょう。",
        "中吉": "積極的な行動が開運につながりそうです。",
        "小吉": "焦らずに行動することが開運につながりそうです。",
        "吉": "穏やかな一日になりそうです。",
        "末吉": "慎重な行動が開運につながりそうです。",
        "凶": "無理せず休むことが開運につながりそうです。",
        "大凶": "ピンチはチャンス、明日に期待しましょう。"
    }

    result = random.choice(fortunes)
    message = messages[result]

    print("=" * 30)
    print("===今日の運勢===")
    print("=" * 30)
    print(f"\n結果：【 {result} 】\n")
    print(f"メッセージ：{message}\n")
    print("=" * 30)

def main():
    print("おみくじプログラム\n")

    while True:
        input("Enterキーを押しておみくじを引きます")
        omikuji()

        retry = input("\nもう一度引きますか？(Y/N)：")
        if retry.lower() != 'y':
            print("\n良い1日を")
            break

if __name__ == "__main__":
    main()