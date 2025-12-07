//おみくじプログラム(C++)

#include<iostream>
#include<string>
#include<random>

using namespace std;

int main() {
	cout << "====おみくじ===" << endl;
	cout << "Enterキーを押しておみくじが引けます";
	cin.get();

	random_device rd;
	mt19937 gen(rd());
	uniform_int_distribution<> dis(1, 5);

	int result = dis(gen);
	string fortune;
	string message;

	switch (result) {
	
		case 1:
			fortune = "大吉";
			message = "最高の1日になりそうです";
			break;

		case 2:
			fortune = "中吉";
			message = "いい感じの1日になりそうです";
			break;

		case 3:
			fortune = "小吉";
			message = "まあまあな1日になりそうです";
			break;

		case 4:
			fortune = "末吉";
			message = "後半に運気が上がりそうです";
			break;

		case 5:
			fortune = "凶";
			message = "明日に期待しましょう";
			break;
	}

	cout << "\n【結果】" << fortune << endl;
	cout << message << endl;
	cout << "\nラッキーナンバー：" << (result * 7) % 10 << endl;

	return 0;
}