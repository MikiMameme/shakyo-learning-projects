#include <iostream>

int main() {
    double height, weight, bmi;

    std::cout << "===BMI計算機===" << std::endl;
    std::cout << "身長を入力してください（単位：cm）";
    std::cin >> height;

    std::cout << "体重を入力してください（単位：kg）";
    std::cin >> weight;

    height = height / 100.0;
    bmi = weight / (height * height);

    std::cout << "\nあなたのBMI： " << bmi << std::endl;
    std::cout << "判定：";

    if (bmi < 18.5) {
        std::cout << "やせ型です" << std::endl;
    } else if (bmi < 25.0) {
        std::cout << "標準です" << std::endl;
    } else if (bmi < 30.0) {
        std::cout << "軽度の肥満です" << std::endl;
    } else {
        std::cout << "重度の肥満です" << std::endl;
    }

    return 0;
}
