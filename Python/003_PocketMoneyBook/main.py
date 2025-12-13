#お小遣い帳アプリ

from datetime import datetime

def show_menu():
    print("\n===お小遣い帳===")
    print("(1)収支を記録")
    print("(2)支出を記録")
    print("(3)履歴を表示")
    print("(4)残高を表示")
    print("(5)終了")
    print("=" * 23)

def add_income(records):
    amount = input("収入額を入力：")
    memo = input("メモ（任意）:")

    try:
        amount = int(amount)
        record = {
            "type" : "収入",
            "amount" : amount,
            "memo" : memo,
            "date" : datetime.now().strftime("%Y-%m-%d %H:%M")
        }
        records.append(record)
        print(f"☑️{amount}円の収入を記録しました")
    except ValueError:
        print("⚠️エラー：数字を入力してください")

def add_expense(records):
    amount = input("支出額を入力：")
    memo = input("メモ（任意）")

    try:
        amount = int(amount)
        record = {
            "type": "支出",
            "amount": amount,
            "memo": memo,
            "date": datetime.now().strftime("%Y-%m-%d %H:%M")
        }
        records.append(record)
        print(f"☑️{amount}円の支出の記録しました")
    except ValueError:
        print("⚠️エラー：数字を入力してください")

def show_history(records):
    if not records:
        print("\n記録がありません")
        return

    print("\n===履歴===")
    for i, record in enumerate(records, 1):
        mark = "+" if record["type"] == "収入" else "-"
        memo_text = f"({record['memo']})1" if record['memo'] else ""
        print(f"{i}.{mark} {record['type']}: {record['amount']:,}円{memo_text}")
        print(f"    日時：{record['date']}")
    print("=" * 30)

def show_balance(records):
    income = sum(r["amount"] for r in records if r ["type"] == "収入")
    expence = sum(r["amount"] for r in records if r ["type"] == "支出")
    balance = income - expence

    print("\n===残高===")
    print(f"収入合計：{income:,}円")
    print(f"支出合計：{expence:,}円")
    print(f"残高：{balance:,}円")
    print("=" * 20)

def main():
    records = []
    print("📝お小遣い帳")

    while True:
        show_menu()
        choice = input("\n選択：")

        if choice == "1":
            add_income(records)
        elif choice == "2":
            add_expense(records)
        elif choice == "3":
            show_history(records)
        elif choice == "4":
            show_balance(records)
        elif choice == "5":
            print("\n終了します。お疲れ様でした")
            break
        else:
            print("エラー：１〜５の数字を入力してください")

if __name__ == "__main__":
    main()
