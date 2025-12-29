#簡易的な電卓アプリ

def add(a, b):
    """足し算"""
    return a + b

def subtract(a, b):
    """引き算"""
    return a - b

def multiply(a, b):
    """掛け算"""
    return a * b

def devide(a, b):
    """割り算"""
    if b == 0:
        return "エラー:ゼロで割ることはできません"
    return a / b

def calculator():
    """電卓のメイン処理"""
    print("=" * 30)
    print("   シンプル電卓")
    print("=" * 30)

    #演算子と関数の対応表
    operations = {
        '+': add,
        '-': subtract,
        '*': multiply,
        '/': devide
    }

    while True:
        print("\n演算子を選んでください：")
        print("  +:足し算")
        print("  -:引き算")
        print("  *:掛け算")
        print("  /:割り算")
        print("  q:終了")

        operator = input("> ").strip()

        if operator == 'q':
            print("\n終了します")
            break

        if operator not in operations:
            print("エラー：無効な演算子です")
            continue

        try:
            num1 = float(input("１つ目の数値："))
            num2 = float(input("２つ目の数値："))

            #辞書から関数を取り出して実行
            result = operations[operator](num1, num2)

            print(f"\n結果：{num1}{operator}{num2} = {result}")

        except ValueError:
            print("エラー：数値を入力してください")

if __name__ == "__main__":
    calculator()