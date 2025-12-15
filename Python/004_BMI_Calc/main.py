#BMI計算機

def calculate_bmi(weight, height):
    bmi = weight / (height ** 2)
    return round(bmi, 1)

def judge_bmi(bmi):
    if bmi < 18.5:
        return "痩せ型","痩せすぎは肥満と同じ「健康上のリスク」があります"
    elif bmi <25:
        return "標準","理想的な状態です"
    elif bmi <30:
        return "肥満(軽度)","運動や食事管理をお勧めします"
    else:
        return "肥満(重度)","運動や食事管理、医師への相談をお勧めします"

def main():
    print("=" * 35)
    print("  BMI計算アプリ")
    print("=" * 35)

    try:
        weight = float(input("体重を入力してください(kg)（例：60）："))
        height = float(input("身長を入力してください(m)（例：1.6）："))

        if weight <= 0 or height <= 0:
            print("エラー:正の数を入力してください")
            return

        bmi = calculate_bmi(weight, height)
        category, advice = judge_bmi(bmi)

        print("\n" + "=" * 35)
        print(f"あなたのBMI：{bmi}")
        print(f"判定：{category}")
        print(f"{advice}")
        print("=" * 35)

    except ValueError:
        print("エラー：数値を入力してください")

if __name__ == "__main__":
    main()